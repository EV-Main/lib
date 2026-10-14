//! The front half of the code flow: a PKCE verifier and its S256 challenge, a `state`, and
//! the URL that sends the browser to concierge's `GET /auth/authorize`.
//!
//! Where these live between the redirect and the callback (a sealed pre-login cookie, a
//! server-side row) is the relying party's call; this module only makes them.

use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD};
use sha2::{Digest, Sha256};

use super::ConciergeError;

/// 32 random bytes, the RFC 7636 recommendation: 43 base64url characters, inside its
/// 43..=128 bound.
const VERIFIER_BYTES: usize = 32;
const STATE_BYTES: usize = 32;

/// The secret half of PKCE. Kept server-side until the callback presents it to
/// [`ConciergeClient::exchange_code`](super::ConciergeClient::exchange_code); only its
/// [`challenge`](Self::challenge) goes to the browser.
#[derive(Clone)]
pub struct PkceVerifier(String);

impl PkceVerifier {
	pub fn generate() -> Result<Self, ConciergeError> {
		Ok(Self(random_b64("a PKCE verifier", VERIFIER_BYTES)?))
	}

	/// A verifier read back from wherever the relying party kept it between the redirect and
	/// the callback.
	pub fn from_stored(verifier: impl Into<String>) -> Self {
		Self(verifier.into())
	}

	pub fn as_str(&self) -> &str {
		&self.0
	}

	/// `BASE64URL(SHA256(verifier))`, unpadded — the only method concierge accepts.
	pub fn challenge(&self) -> String {
		URL_SAFE_NO_PAD.encode(Sha256::digest(self.0.as_bytes()))
	}
}

impl std::fmt::Debug for PkceVerifier {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_str("PkceVerifier(<redacted>)")
	}
}

/// An unguessable `state` for the authorize redirect. The callback compares it with the
/// stored one in constant time and refuses a state it has seen before.
pub fn random_state() -> Result<String, ConciergeError> {
	random_b64("an authorize state", STATE_BYTES)
}

fn random_b64(what: &'static str, len: usize) -> Result<String, ConciergeError> {
	let mut bytes = vec![0u8; len];
	getrandom::fill(&mut bytes).map_err(|source| ConciergeError::Randomness { what, source })?;
	Ok(URL_SAFE_NO_PAD.encode(bytes))
}

/// Ask concierge to show its account picker even when the browser is already signed in.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Prompt {
	SelectAccount,
}

/// Everything `GET /auth/authorize` takes. `redirect_uri` must be byte for byte the one
/// later passed to `exchange_code`, and registered for the client at concierge.
#[derive(Clone, Debug)]
pub struct Authorize<'a> {
	/// The public origin concierge's web surface answers on, without a trailing slash
	/// (`https://evinvest.ltd`); the path `/api/auth/authorize` is appended.
	pub concierge_origin: &'a str,
	pub client_id: &'a str,
	pub redirect_uri: &'a str,
	pub state: &'a str,
	pub verifier: &'a PkceVerifier,
	pub prompt: Option<Prompt>,
}

impl Authorize<'_> {
	pub fn url(&self) -> String {
		let mut url = format!(
			"{}/api/auth/authorize?response_type=code&client_id={}&redirect_uri={}&state={}&code_challenge={}&code_challenge_method=S256",
			self.concierge_origin.trim_end_matches('/'),
			encode(self.client_id),
			encode(self.redirect_uri),
			encode(self.state),
			self.verifier.challenge(),
		);
		match self.prompt {
			Some(Prompt::SelectAccount) => url.push_str("&prompt=select_account"),
			None => {}
		}
		url
	}
}

/// RFC 3986 query-component encoding: everything but the unreserved set is `%XX`.
fn encode(value: &str) -> String {
	let mut out = String::with_capacity(value.len());
	for byte in value.bytes() {
		match byte {
			b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'.' | b'_' | b'~' => out.push(char::from(byte)),
			_ => out.push_str(&format!("%{byte:02X}")),
		}
	}
	out
}
