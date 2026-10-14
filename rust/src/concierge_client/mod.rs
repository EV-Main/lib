//! `concierge_client` — what a relying party (a direction's backend: invest, admin, the
//! Service-Arb panel) calls on concierge, the identity plane.
//!
//! - Sign-in is concierge's code flow with PKCE: [`PkceVerifier`] + [`random_state`] +
//!   [`Authorize::url`] send the browser to `/api/auth/authorize`; the callback redeems the
//!   code with [`ConciergeClient::exchange_code`]; [`ConciergeClient::refresh_client_token`]
//!   rotates the pair.
//! - [`ConciergeClient::get_me`] — the one RPC a relying party's access token opens. It is
//!   also where the party re-reads `permissions`, the only thing it may gate on.
//! - [`ConciergeClient::publish_catalog`] (permissions and aliases) and
//!   [`ConciergeClient::publish_scope_catalog`] (approval kinds and notification topics).
//!
//! The client secret authenticates the server-to-server calls: in the request body for
//! the `AuthService` ones (their wire says so), as `x-ev-client-id` + `authorization:
//! Bearer` metadata for `ScopeCatalogService` (docs/SCOPES-AND-APPROVALS.md §2 of
//! concierge). It never reaches a browser.
//!
//! The wire types are concierge's own `evconcierge_contracts`, re-exported as [`proto`]
//! for the messages this module takes as they are (a catalog's aliases, approval kind
//! specs).
//!
//! ```ignore
//! let concierge = ConciergeClient::connect_lazy(ClientConfig::new(grpc_addr, "sa", secret))?;
//! let verifier = PkceVerifier::generate()?;
//! let state = random_state()?;
//! let location = Authorize { concierge_origin, client_id: "sa", redirect_uri, state: &state, verifier: &verifier, prompt: None }.url();
//! // … the browser comes back with `code` …
//! let tokens = concierge.exchange_code(&code, redirect_uri, &verifier).await?;
//! let me = concierge.get_me(&tokens.access_token).await?;
//! ```

mod error;
mod pkce;

use std::time::Duration;

pub use error::ConciergeError;
pub use evconcierge_contracts::concierge::v1 as proto;
pub use pkce::{Authorize, PkceVerifier, Prompt, random_state};
use proto::{
	CatalogAlias, ClientTokenResponse, ExchangeCodeRequest, GetMeRequest, PublishCatalogRequest, PublishScopeCatalogRequest, RefreshClientTokenRequest, UserProfile,
	auth_service_client::AuthServiceClient, scope_catalog_service_client::ScopeCatalogServiceClient, user_directory_client::UserDirectoryClient,
};
use tonic_concierge::{
	Request,
	metadata::{AsciiMetadataValue, MetadataValue},
	transport::{Channel, Endpoint},
};
use uuid::Uuid;

/// How long one call may take, connecting included, unless [`ClientConfig`] says otherwise.
/// A sign-in callback is waiting on it, so seconds, not a minute.
pub const DEFAULT_TIMEOUT: Duration = Duration::from_secs(5);

/// Where concierge is and who this relying party is to it.
#[derive(Clone)]
pub struct ClientConfig {
	/// concierge's gRPC address (`http://concierge:55670`).
	pub endpoint: String,
	pub client_id: String,
	pub client_secret: String,
	pub timeout: Duration,
}

impl ClientConfig {
	pub fn new(endpoint: impl Into<String>, client_id: impl Into<String>, client_secret: impl Into<String>) -> Self {
		Self {
			endpoint: endpoint.into(),
			client_id: client_id.into(),
			client_secret: client_secret.into(),
			timeout: DEFAULT_TIMEOUT,
		}
	}
}

impl std::fmt::Debug for ClientConfig {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("ClientConfig")
			.field("endpoint", &self.endpoint)
			.field("client_id", &self.client_id)
			.field("client_secret", &"<redacted>")
			.field("timeout", &self.timeout)
			.finish()
	}
}

/// A relying party's token pair, as `ExchangeCode` and `RefreshClientToken` answer it.
/// Expiries are unix seconds. The access token opens `GetMe` and nothing else; the refresh
/// token is bound to this client and rotates on every use.
#[derive(Clone)]
pub struct ClientTokens {
	pub user_id: Uuid,
	pub access_token: String,
	pub access_expires_at: i64,
	pub refresh_token: String,
	pub refresh_expires_at: i64,
}

impl std::fmt::Debug for ClientTokens {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("ClientTokens")
			.field("user_id", &self.user_id)
			.field("access_expires_at", &self.access_expires_at)
			.field("refresh_expires_at", &self.refresh_expires_at)
			.finish_non_exhaustive()
	}
}

/// The signed-in user as `GetMe` tells a relying party. Deliberately narrower than the
/// wire's `UserProfile`: the identity-document fields (legal name, date of birth, address,
/// nationality, tax residence, phone) and the KYC level are not the direction's to hold,
/// and concierge leaves them empty for a relying party anyway.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Me {
	pub user_id: Uuid,
	pub email: String,
	pub email_verified: bool,
	/// Empty when the account has none.
	pub username: String,
	pub preferred_name: String,
	pub status: String,
	pub token_version: u64,
	/// Concrete permissions in this relying party's namespace: no alias, no wildcard. What
	/// the party gates on, and nothing else.
	pub permissions: Vec<String>,
}

impl TryFrom<UserProfile> for Me {
	type Error = ConciergeError;

	fn try_from(p: UserProfile) -> Result<Self, ConciergeError> {
		Ok(Self {
			user_id: parse_user_id(&p.user_id, "GetMe")?,
			email: p.email,
			email_verified: p.email_verified,
			username: p.username,
			preferred_name: p.preferred_name,
			status: p.status,
			token_version: p.token_version,
			permissions: p.permissions,
		})
	}
}

/// A tenant's catalog for `PublishCatalog`: the permissions and aliases its namespace
/// defines.
#[derive(Clone, Debug, Default)]
pub struct Catalog {
	/// Unix seconds of the commit the service was built from (nix `self.lastModified`):
	/// build time, never boot time, so a restarting older replica cannot supersede a newer
	/// one.
	pub version: u64,
	pub permissions: Vec<String>,
	pub aliases: Vec<CatalogAlias>,
}

/// What `PublishScopeCatalog` takes: the namespace's approval kinds and notification topics,
/// whole — a kind or topic left out is withdrawn.
#[derive(Clone, Debug, Default)]
pub struct ScopeCatalog {
	/// Same rule as [`Catalog::version`].
	pub version: u64,
	pub approval_kinds: Vec<proto::ApprovalKindSpec>,
	pub notification_topics: Vec<proto::NotificationTopicSpec>,
}

/// A cheaply cloneable handle on concierge for one relying party. Connects on first use, so
/// the service boots (and serves what does not need concierge) while concierge is down.
#[derive(Clone)]
pub struct ConciergeClient {
	auth: AuthServiceClient<Channel>,
	directory: UserDirectoryClient<Channel>,
	scope_catalog: ScopeCatalogServiceClient<Channel>,
	client_id: String,
	client_secret: String,
	client_id_header: AsciiMetadataValue,
	bearer_header: AsciiMetadataValue,
}

impl std::fmt::Debug for ConciergeClient {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("ConciergeClient").field("client_id", &self.client_id).finish_non_exhaustive()
	}
}

impl ConciergeClient {
	pub fn connect_lazy(config: ClientConfig) -> Result<Self, ConciergeError> {
		let channel = Endpoint::from_shared(config.endpoint.clone())
			.map_err(|e| ConciergeError::Config(format!("concierge endpoint {:?}: {e}", config.endpoint)))?
			.connect_timeout(config.timeout)
			.timeout(config.timeout)
			.connect_lazy();
		Self::from_channel(channel, config.client_id, config.client_secret)
	}

	/// Over a channel the caller built (its own TLS, its own timeouts).
	pub fn from_channel(channel: Channel, client_id: impl Into<String>, client_secret: impl Into<String>) -> Result<Self, ConciergeError> {
		let client_id = client_id.into();
		let client_secret = client_secret.into();
		let client_id_header = MetadataValue::try_from(client_id.as_str()).map_err(|_| ConciergeError::Config("the client id is not a valid header value".into()))?;
		let bearer_header = MetadataValue::try_from(format!("Bearer {client_secret}")).map_err(|_| ConciergeError::Config("the client secret is not a valid header value".into()))?;
		Ok(Self {
			auth: AuthServiceClient::new(channel.clone()),
			directory: UserDirectoryClient::new(channel.clone()),
			scope_catalog: ScopeCatalogServiceClient::new(channel),
			client_id,
			client_secret,
			client_id_header,
			bearer_header,
		})
	}

	pub fn client_id(&self) -> &str {
		&self.client_id
	}

	/// Redeem the code the browser brought back to `redirect_uri`. The code is single-use and
	/// lives 60 s; presenting it twice is refused and revokes every token issued off it.
	pub async fn exchange_code(&self, code: &str, redirect_uri: &str, verifier: &PkceVerifier) -> Result<ClientTokens, ConciergeError> {
		let answer = self
			.auth
			.clone()
			.exchange_code(ExchangeCodeRequest {
				client_id: self.client_id.clone(),
				client_secret: self.client_secret.clone(),
				code: code.to_owned(),
				redirect_uri: redirect_uri.to_owned(),
				code_verifier: verifier.as_str().to_owned(),
			})
			.await?
			.into_inner();
		tokens(answer, "ExchangeCode")
	}

	/// Rotate the refresh token. [`ConciergeError::Refused`] means the session is over (signed
	/// out on evinvest.ltd, suspended, a rotated-out token presented): drop it.
	pub async fn refresh_client_token(&self, refresh_token: &str) -> Result<ClientTokens, ConciergeError> {
		let answer = self
			.auth
			.clone()
			.refresh_client_token(RefreshClientTokenRequest {
				client_id: self.client_id.clone(),
				client_secret: self.client_secret.clone(),
				refresh_token: refresh_token.to_owned(),
			})
			.await?
			.into_inner();
		tokens(answer, "RefreshClientToken")
	}

	/// The user behind a relying party's access token.
	pub async fn get_me(&self, access_token: &str) -> Result<Me, ConciergeError> {
		let mut request = Request::new(GetMeRequest {});
		let bearer = MetadataValue::try_from(format!("Bearer {access_token}")).map_err(|_| ConciergeError::Config("the access token is not a valid header value".into()))?;
		request.metadata_mut().insert("authorization", bearer);
		self.directory.clone().get_me(request).await?.into_inner().try_into()
	}

	/// Publish the tenant's permissions and aliases. [`ConciergeError::Rejected`] with
	/// FAILED_PRECONDITION is an older (or same-version, different) catalog than the one
	/// concierge holds — the normal answer to an older replica, not an outage.
	pub async fn publish_catalog(&self, catalog: Catalog) -> Result<(), ConciergeError> {
		self.auth
			.clone()
			.publish_catalog(PublishCatalogRequest {
				client_id: self.client_id.clone(),
				client_secret: self.client_secret.clone(),
				version: catalog.version,
				permissions: catalog.permissions,
				aliases: catalog.aliases,
			})
			.await?;
		Ok(())
	}

	/// Publish the namespace's approval kinds and notification topics, whole.
	pub async fn publish_scope_catalog(&self, catalog: ScopeCatalog) -> Result<(), ConciergeError> {
		let request = self.with_credential(PublishScopeCatalogRequest {
			version: catalog.version,
			approval_kinds: catalog.approval_kinds,
			notification_topics: catalog.notification_topics,
		});
		self.scope_catalog.clone().publish_scope_catalog(request).await?;
		Ok(())
	}

	/// The service credential of docs/SCOPES-AND-APPROVALS.md §2: headers, never the body, so
	/// a logged request carries no secret.
	fn with_credential<T>(&self, message: T) -> Request<T> {
		let mut request = Request::new(message);
		let metadata = request.metadata_mut();
		metadata.insert("x-ev-client-id", self.client_id_header.clone());
		metadata.insert("authorization", self.bearer_header.clone());
		request
	}
}

fn tokens(r: ClientTokenResponse, rpc: &str) -> Result<ClientTokens, ConciergeError> {
	Ok(ClientTokens {
		user_id: parse_user_id(&r.user_id, rpc)?,
		access_token: r.access_token,
		access_expires_at: r.access_expires_at,
		refresh_token: r.refresh_token,
		refresh_expires_at: r.refresh_expires_at,
	})
}

fn parse_user_id(raw: &str, rpc: &str) -> Result<Uuid, ConciergeError> {
	Uuid::parse_str(raw).map_err(|_| ConciergeError::InvalidResponse(format!("{rpc} answered a user id that is not a UUID")))
}
