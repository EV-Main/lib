//! `concierge_client`: the PKCE half of the code flow, the error mapping, redaction, and
//! the calls themselves against an in-process fake of concierge's gRPC.
#![cfg(feature = "concierge_client")]

use std::sync::{Arc, Mutex};

use ev_lib::concierge_client::{Authorize, ClientConfig, ClientTokens, ConciergeClient, ConciergeError, Me, PkceVerifier, Prompt, ScopeCatalog, proto, random_state};
use proto::{
	ClientTokenResponse, ExchangeCodeRequest, GetScopeCatalogRequest, JwksRequest, JwksResponse, ListSessionsRequest, ListSessionsResponse, LogoutRequest, LogoutResponse,
	PublishCatalogRequest, PublishCatalogResponse, PublishScopeCatalogRequest, PublishScopeCatalogResponse, RefreshClientTokenRequest, RefreshRequest, RevokeSessionRequest,
	RevokeSessionResponse, TokenResponse, UserProfile,
	auth_service_server::{AuthService, AuthServiceServer},
	scope_catalog_service_server::{ScopeCatalogService, ScopeCatalogServiceServer},
};
use tonic_concierge::{
	Code, Request, Response, Status,
	transport::{Endpoint, Server, server::TcpIncoming},
};
use uuid::Uuid;

/// RFC 7636, Appendix B.
const RFC_VERIFIER: &str = "dBjftJeZ4CVP-mB92K27uhbUJU1p1r_wW1gFWFOEjXk";
const RFC_CHALLENGE: &str = "E9Melhoa2OwvFrEMTJguCHaoeK1t8URWbuGJSstw-cM";

// --- PKCE and the authorize URL -------------------------------------------------------

#[test]
fn challenge_should_match_the_rfc_7636_appendix_b_vector() {
	let verifier = PkceVerifier::from_stored(RFC_VERIFIER);

	assert_eq!(verifier.challenge(), RFC_CHALLENGE);
}

#[test]
fn generated_verifier_should_be_43_base64url_characters() {
	let verifier = PkceVerifier::generate().unwrap();

	assert_eq!(verifier.as_str().len(), 43, "{:?}", verifier.as_str());
	assert!(verifier.as_str().bytes().all(|b| b.is_ascii_alphanumeric() || b == b'-' || b == b'_'), "{:?}", verifier.as_str());
}

#[test]
fn generated_verifiers_should_differ() {
	assert_ne!(PkceVerifier::generate().unwrap().as_str(), PkceVerifier::generate().unwrap().as_str());
}

#[test]
fn random_state_should_be_43_characters_and_differ_each_time() {
	let first = random_state().unwrap();

	assert_eq!(first.len(), 43, "{first:?}");
	assert_ne!(first, random_state().unwrap());
}

#[test]
fn verifier_debug_should_not_print_the_verifier() {
	let shown = format!("{:?}", PkceVerifier::from_stored(RFC_VERIFIER));

	assert!(!shown.contains(RFC_VERIFIER), "{shown}");
}

#[test]
fn authorize_url_should_percent_encode_the_redirect_uri() {
	let verifier = PkceVerifier::from_stored(RFC_VERIFIER);
	let authorize = Authorize {
		concierge_origin: "https://evinvest.ltd",
		client_id: "sa",
		redirect_uri: "https://sa.evinvest.ltd/auth/callback?next=/café b&x=1",
		state: "st~a.te_-",
		verifier: &verifier,
		prompt: None,
	};

	assert_eq!(
		authorize.url(),
		"https://evinvest.ltd/api/auth/authorize?response_type=code&client_id=sa\
		 &redirect_uri=https%3A%2F%2Fsa.evinvest.ltd%2Fauth%2Fcallback%3Fnext%3D%2Fcaf%C3%A9%20b%26x%3D1\
		 &state=st~a.te_-&code_challenge=E9Melhoa2OwvFrEMTJguCHaoeK1t8URWbuGJSstw-cM&code_challenge_method=S256"
	);
}

#[test]
fn authorize_url_should_ask_for_the_account_picker_when_prompted() {
	let verifier = PkceVerifier::from_stored(RFC_VERIFIER);
	let authorize = Authorize {
		concierge_origin: "https://evinvest.ltd",
		client_id: "sa",
		redirect_uri: "https://sa.evinvest.ltd/cb",
		state: "st",
		verifier: &verifier,
		prompt: Some(Prompt::SelectAccount),
	};

	assert_eq!(
		authorize.url(),
		"https://evinvest.ltd/api/auth/authorize?response_type=code&client_id=sa&redirect_uri=https%3A%2F%2Fsa.evinvest.ltd%2Fcb\
		 &state=st&code_challenge=E9Melhoa2OwvFrEMTJguCHaoeK1t8URWbuGJSstw-cM&code_challenge_method=S256&prompt=select_account"
	);
}

#[test]
fn authorize_url_should_not_double_the_slash_after_an_origin_that_ends_with_one() {
	let verifier = PkceVerifier::from_stored(RFC_VERIFIER);
	let authorize = Authorize {
		concierge_origin: "https://evinvest.ltd/",
		client_id: "sa",
		redirect_uri: "https://sa.evinvest.ltd/cb",
		state: "st",
		verifier: &verifier,
		prompt: None,
	};

	let url = authorize.url();

	assert!(url.starts_with("https://evinvest.ltd/api/auth/authorize?response_type=code&"), "{url}");
}

// --- errors -------------------------------------------------------------------------------

#[test]
fn unauthenticated_and_permission_denied_should_be_refused() {
	let unauthenticated = ConciergeError::from(Status::unauthenticated("wrong client secret"));
	let denied = ConciergeError::from(Status::permission_denied("not yours"));

	assert!(matches!(unauthenticated, ConciergeError::Refused { code: Code::Unauthenticated }), "{unauthenticated:?}");
	assert!(matches!(denied, ConciergeError::Refused { code: Code::PermissionDenied }), "{denied:?}");
	assert!(!unauthenticated.is_transient());
}

#[test]
fn invalid_argument_and_failed_precondition_should_be_rejected_with_concierge_message() {
	let invalid = ConciergeError::from(Status::invalid_argument("sa.x is outside the namespace"));
	let older = ConciergeError::from(Status::failed_precondition("catalog version 3 is older than 5"));

	assert!(
		matches!(&invalid, ConciergeError::Rejected { code: Code::InvalidArgument, message } if message == "sa.x is outside the namespace"),
		"{invalid:?}"
	);
	assert!(
		matches!(&older, ConciergeError::Rejected { code: Code::FailedPrecondition, message } if message == "catalog version 3 is older than 5"),
		"{older:?}"
	);
	assert!(!older.is_transient());
}

#[test]
fn unavailable_deadline_and_cancelled_should_be_unavailable_and_transient() {
	let unavailable = ConciergeError::from(Status::unavailable("connection refused"));
	let deadline = ConciergeError::from(Status::deadline_exceeded("timed out"));
	let cancelled = ConciergeError::from(Status::cancelled("cancelled"));

	assert!(matches!(&unavailable, ConciergeError::Unavailable(m) if m == "connection refused"), "{unavailable:?}");
	assert!(matches!(deadline, ConciergeError::Unavailable(_)), "{deadline:?}");
	assert!(matches!(cancelled, ConciergeError::Unavailable(_)), "{cancelled:?}");
	assert!(unavailable.is_transient());
}

#[test]
fn any_other_status_should_be_failed_and_not_transient() {
	let internal = ConciergeError::from(Status::internal("boom"));
	let not_found = ConciergeError::from(Status::not_found("no such user"));
	let unimplemented = ConciergeError::from(Status::unimplemented("older concierge"));

	assert!(matches!(&internal, ConciergeError::Failed { code: Code::Internal, message } if message == "boom"), "{internal:?}");
	assert!(matches!(not_found, ConciergeError::Failed { code: Code::NotFound, .. }), "{not_found:?}");
	assert!(matches!(unimplemented, ConciergeError::Failed { code: Code::Unimplemented, .. }), "{unimplemented:?}");
	assert!(!internal.is_transient());
}

// --- redaction ---------------------------------------------------------------------------

#[test]
fn client_tokens_debug_should_not_print_either_token() {
	let tokens = ClientTokens {
		user_id: Uuid::nil(),
		access_token: "access-SECRET-1".into(),
		access_expires_at: 1_700_000_000,
		refresh_token: "refresh-SECRET-2".into(),
		refresh_expires_at: 1_700_086_400,
	};

	let shown = format!("{tokens:?}");

	assert!(!shown.contains("access-SECRET-1"), "{shown}");
	assert!(!shown.contains("refresh-SECRET-2"), "{shown}");
	assert!(shown.contains("00000000-0000-0000-0000-000000000000"), "{shown}");
}

#[test]
fn client_config_debug_should_not_print_the_client_secret() {
	let shown = format!("{:?}", ClientConfig::new("http://concierge:55670", "sa", "client-SECRET"));

	assert!(!shown.contains("client-SECRET"), "{shown}");
	assert!(shown.contains("sa"), "{shown}");
}

#[tokio::test]
async fn concierge_client_debug_should_not_print_the_client_secret() {
	let channel = Endpoint::from_static("http://127.0.0.1:9").connect_lazy();
	let client = ConciergeClient::from_channel(channel, "sa", "client-SECRET").unwrap();

	let shown = format!("{client:?}");

	assert!(!shown.contains("client-SECRET"), "{shown}");
}

#[tokio::test]
async fn a_secret_that_cannot_travel_as_a_header_should_be_a_config_error() {
	let channel = Endpoint::from_static("http://127.0.0.1:9").connect_lazy();

	let error = ConciergeClient::from_channel(channel, "sa", "line\nbreak").unwrap_err();

	assert!(matches!(error, ConciergeError::Config(_)), "{error:?}");
}

#[test]
fn an_endpoint_that_is_not_a_uri_should_be_a_config_error() {
	let error = ConciergeClient::connect_lazy(ClientConfig::new("not a uri", "sa", "secret")).unwrap_err();

	assert!(matches!(error, ConciergeError::Config(_)), "{error:?}");
}

// --- GetMe's profile ----------------------------------------------------------------------

#[test]
fn get_me_profile_with_a_user_id_that_is_not_a_uuid_should_be_an_invalid_response() {
	let profile = UserProfile {
		user_id: "user-42".into(),
		..UserProfile::default()
	};

	let error = Me::try_from(profile).unwrap_err();

	assert!(matches!(error, ConciergeError::InvalidResponse(_)), "{error:?}");
}

#[test]
fn get_me_profile_should_keep_the_permissions_as_concierge_sent_them() {
	let profile = UserProfile {
		user_id: "6f1c8a4e-2b0d-4c61-9b8e-0a1d2c3e4f50".into(),
		email: "ann@example.com".into(),
		permissions: vec!["sa.leads.read".into(), "sa.places.write".into()],
		..UserProfile::default()
	};

	let me = Me::try_from(profile).unwrap();

	assert_eq!(me.user_id, Uuid::parse_str("6f1c8a4e-2b0d-4c61-9b8e-0a1d2c3e4f50").unwrap());
	assert_eq!(me.permissions, vec!["sa.leads.read".to_owned(), "sa.places.write".to_owned()]);
}

// --- the calls, against a fake concierge --------------------------------------------------

/// A `PublishScopeCatalog` as the fake saw it: `x-ev-client-id`, `authorization`, `version`.
type ScopePublish = (Option<String>, Option<String>, u64);

/// What the fake concierge answers and what it was sent.
#[derive(Clone, Default)]
struct FakeConcierge {
	answer_user_id: Arc<Mutex<String>>,
	refuse: Arc<Mutex<bool>>,
	exchanged: Arc<Mutex<Vec<ExchangeCodeRequest>>>,
	scope_publishes: Arc<Mutex<Vec<ScopePublish>>>,
}

impl FakeConcierge {
	fn answering(user_id: &str) -> Self {
		let fake = Self::default();
		*fake.answer_user_id.lock().unwrap() = user_id.to_owned();
		fake
	}

	fn token_response(&self) -> Result<Response<ClientTokenResponse>, Status> {
		if *self.refuse.lock().unwrap() {
			return Err(Status::unauthenticated("refresh family revoked"));
		}
		Ok(Response::new(ClientTokenResponse {
			user_id: self.answer_user_id.lock().unwrap().clone(),
			access_token: "access".into(),
			access_expires_at: 1_700_000_900,
			refresh_token: "refresh".into(),
			refresh_expires_at: 1_702_592_000,
		}))
	}
}

#[async_trait::async_trait]
impl AuthService for FakeConcierge {
	async fn refresh(&self, _: Request<RefreshRequest>) -> Result<Response<TokenResponse>, Status> {
		Err(Status::unimplemented("not in this fake"))
	}

	async fn logout(&self, _: Request<LogoutRequest>) -> Result<Response<LogoutResponse>, Status> {
		Err(Status::unimplemented("not in this fake"))
	}

	async fn list_sessions(&self, _: Request<ListSessionsRequest>) -> Result<Response<ListSessionsResponse>, Status> {
		Err(Status::unimplemented("not in this fake"))
	}

	async fn revoke_session(&self, _: Request<RevokeSessionRequest>) -> Result<Response<RevokeSessionResponse>, Status> {
		Err(Status::unimplemented("not in this fake"))
	}

	async fn jwks(&self, _: Request<JwksRequest>) -> Result<Response<JwksResponse>, Status> {
		Err(Status::unimplemented("not in this fake"))
	}

	async fn exchange_code(&self, request: Request<ExchangeCodeRequest>) -> Result<Response<ClientTokenResponse>, Status> {
		self.exchanged.lock().unwrap().push(request.into_inner());
		self.token_response()
	}

	async fn refresh_client_token(&self, _: Request<RefreshClientTokenRequest>) -> Result<Response<ClientTokenResponse>, Status> {
		self.token_response()
	}

	async fn publish_catalog(&self, _: Request<PublishCatalogRequest>) -> Result<Response<PublishCatalogResponse>, Status> {
		Err(Status::failed_precondition("catalog version 3 is older than 5"))
	}
}

#[async_trait::async_trait]
impl ScopeCatalogService for FakeConcierge {
	async fn publish_scope_catalog(&self, request: Request<PublishScopeCatalogRequest>) -> Result<Response<PublishScopeCatalogResponse>, Status> {
		let header = |name: &str| request.metadata().get(name).map(|v| v.to_str().unwrap().to_owned());
		let seen = (header("x-ev-client-id"), header("authorization"), request.get_ref().version);
		self.scope_publishes.lock().unwrap().push(seen);
		Ok(Response::new(PublishScopeCatalogResponse::default()))
	}

	async fn get_scope_catalog(&self, _: Request<GetScopeCatalogRequest>) -> Result<Response<proto::ScopeCatalog>, Status> {
		Err(Status::unimplemented("not in this fake"))
	}
}

/// Serve `fake` on a loopback port and return a client pointed at it.
async fn serve(fake: FakeConcierge) -> ConciergeClient {
	let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
	let addr = listener.local_addr().unwrap();
	tokio::spawn(
		Server::builder()
			.add_service(AuthServiceServer::new(fake.clone()))
			.add_service(ScopeCatalogServiceServer::new(fake))
			.serve_with_incoming(TcpIncoming::from(listener)),
	);
	ConciergeClient::connect_lazy(ClientConfig::new(format!("http://{addr}"), "sa", "client-SECRET")).unwrap()
}

#[tokio::test]
async fn exchange_code_should_present_the_code_redirect_uri_verifier_and_client_credential() {
	let fake = FakeConcierge::answering("6f1c8a4e-2b0d-4c61-9b8e-0a1d2c3e4f50");
	let client = serve(fake.clone()).await;

	let tokens = client
		.exchange_code("the-code", "https://sa.evinvest.ltd/cb", &PkceVerifier::from_stored(RFC_VERIFIER))
		.await
		.unwrap();

	assert_eq!(tokens.user_id, Uuid::parse_str("6f1c8a4e-2b0d-4c61-9b8e-0a1d2c3e4f50").unwrap());
	assert_eq!(
		fake.exchanged.lock().unwrap().as_slice(),
		[ExchangeCodeRequest {
			client_id: "sa".into(),
			client_secret: "client-SECRET".into(),
			code: "the-code".into(),
			redirect_uri: "https://sa.evinvest.ltd/cb".into(),
			code_verifier: RFC_VERIFIER.into(),
		}]
	);
}

#[tokio::test]
async fn exchange_code_should_be_an_invalid_response_when_concierge_answers_a_user_id_that_is_not_a_uuid() {
	let client = serve(FakeConcierge::answering("user-42")).await;

	let error = client
		.exchange_code("the-code", "https://sa.evinvest.ltd/cb", &PkceVerifier::from_stored(RFC_VERIFIER))
		.await
		.unwrap_err();

	assert!(matches!(&error, ConciergeError::InvalidResponse(m) if m.contains("ExchangeCode")), "{error:?}");
}

#[tokio::test]
async fn refresh_client_token_should_be_refused_when_concierge_says_unauthenticated() {
	let fake = FakeConcierge::answering("6f1c8a4e-2b0d-4c61-9b8e-0a1d2c3e4f50");
	*fake.refuse.lock().unwrap() = true;
	let client = serve(fake).await;

	let error = client.refresh_client_token("rotated-out").await.unwrap_err();

	assert!(matches!(error, ConciergeError::Refused { code: Code::Unauthenticated }), "{error:?}");
}

#[tokio::test]
async fn publish_catalog_should_be_rejected_with_failed_precondition_for_an_older_version() {
	let client = serve(FakeConcierge::default()).await;

	let error = client.publish_catalog(ev_lib::concierge_client::Catalog::default()).await.unwrap_err();

	assert!(matches!(error, ConciergeError::Rejected { code: Code::FailedPrecondition, .. }), "{error:?}");
}

#[tokio::test]
async fn publish_scope_catalog_should_carry_the_client_credential_as_headers() {
	let fake = FakeConcierge::default();
	let client = serve(fake.clone()).await;

	client
		.publish_scope_catalog(ScopeCatalog {
			version: 1_760_000_000,
			..ScopeCatalog::default()
		})
		.await
		.unwrap();

	assert_eq!(
		fake.scope_publishes.lock().unwrap().as_slice(),
		[(Some("sa".to_owned()), Some("Bearer client-SECRET".to_owned()), 1_760_000_000)]
	);
}

#[tokio::test]
async fn a_call_to_a_concierge_nobody_listens_on_should_be_unavailable() {
	let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
	let addr = listener.local_addr().unwrap();
	drop(listener);
	let client = ConciergeClient::connect_lazy(ClientConfig::new(format!("http://{addr}"), "sa", "secret")).unwrap();

	let error = client.refresh_client_token("r").await.unwrap_err();

	assert!(matches!(error, ConciergeError::Unavailable(_)), "{error:?}");
	assert!(error.is_transient());
}
