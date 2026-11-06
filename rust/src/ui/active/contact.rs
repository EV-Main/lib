//! Outbound contact links — the conversions a landing exists for. The Rust twin
//! of `@evinvest/marketing`'s `telHref` / `whatsappHref` / `contactChannel`, so
//! a Rust page links the same number the same way and a click tracker
//! classifies it the same way. Both are pinned to `tests/fixtures/marketing/contact.json`.

// `web.whatsapp.com/send` is where desktop "click to chat" lands.
pub const WHATSAPP_HOSTS: [&str; 3] = ["wa.me", "api.whatsapp.com", "web.whatsapp.com"];
/// The channel a contact link opens.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, strum::EnumIter)]
pub enum ContactChannel {
	Phone,
	Whatsapp,
	Email,
}

impl ContactChannel {
	/// The analytics value: `"phone"`, `"whatsapp"`, `"email"`.
	pub fn as_str(self) -> &'static str {
		match self {
			Self::Phone => "phone",
			Self::Whatsapp => "whatsapp",
			Self::Email => "email",
		}
	}
}

// `whatsapp://send` is the native deep link some pages use directly.
/// Which contact channel `href` opens, or `None` when it is not a contact link.
/// Relative hrefs are never a contact link.
///
/// ```
/// use ev_lib::marketing::{ContactChannel, contact_channel};
/// assert_eq!(contact_channel("tel:+33423500640"), Some(ContactChannel::Phone));
/// assert_eq!(contact_channel("https://wa.me/33423500640"), Some(ContactChannel::Whatsapp));
/// assert_eq!(contact_channel("/fr/prices"), None);
/// assert_eq!(contact_channel("https://wa%2Eme\\33423500640"), Some(ContactChannel::Whatsapp));
/// assert_eq!(contact_channel("https://wa.me.evil.example/33"), None);
/// ```
pub fn contact_channel(href: &str) -> Option<ContactChannel> {
	let value = href.trim().to_lowercase();
	if value.starts_with("tel:") {
		return Some(ContactChannel::Phone);
	}
	if value.starts_with("mailto:") {
		return Some(ContactChannel::Email);
	}
	if value.starts_with("whatsapp:") {
		return Some(ContactChannel::Whatsapp);
	}
	let rest = value.strip_prefix("https://").or_else(|| value.strip_prefix("http://"))?;
	WHATSAPP_HOSTS.contains(&url_host(rest).as_str()).then_some(ContactChannel::Whatsapp)
}

/// `tel:` href from a number as printed. RFC 3966 allows only digits, `+` and
/// visual separators; keeping digits and `+` (and dropping a `(0)` trunk
/// prefix) keeps "04 23 50 06 40" and "+33 (0)4…" dialable on every phone.
///
/// ```
/// use ev_lib::marketing::tel_href;
/// assert_eq!(tel_href("+33 (0)4 23 50 06 40"), "tel:+33423500640");
/// ```
pub fn tel_href(phone: &str) -> String {
	let digits: String = phone.replace("(0)", "").chars().filter(|c| c.is_ascii_digit() || *c == '+').collect();
	format!("tel:{digits}")
}
/// `wa.me` click-to-chat href. wa.me wants the international number as bare
/// digits — no `+`, no `00` exit prefix — and answers anything else with a
/// generic "invalid link" page. A national number ("06 12 …") cannot be fixed
/// without its country, so it is left as it is and the broken link is visible
/// rather than silently dialling someone else.
///
/// ```
/// use ev_lib::marketing::whatsapp_href;
/// assert_eq!(whatsapp_href("0033 4 23 50 06 40", Some("Bonjour !")), "https://wa.me/33423500640?text=Bonjour%20!");
/// ```
pub fn whatsapp_href(phone: &str, message: Option<&str>) -> String {
	let digits: String = phone.replace("(0)", "").chars().filter(char::is_ascii_digit).collect();
	let digits = digits.strip_prefix("00").unwrap_or(&digits);
	match message.filter(|m| !m.is_empty()) {
		Some(text) => format!("https://wa.me/{digits}?text={}", encode_uri_component(text)),
		None => format!("https://wa.me/{digits}"),
	}
}
/// The host of an `http(s)` URL after its `//`, as the WHATWG URL parser —
/// what the TypeScript port asks — reads it: tabs and newlines dropped, `\`
/// ending the authority like `/`, userinfo and port cut, percent-escapes
/// decoded. `https://wa.me\x`, `https://u@wa.me:443` and `https://wa%2Eme`
/// all name `wa.me` to a browser, so they do here.
fn url_host(after_scheme: &str) -> String {
	let cleaned: String = after_scheme.chars().filter(|c| !matches!(c, '\t' | '\n' | '\r')).collect();
	let cleaned = cleaned.trim_start_matches(['/', '\\']);
	let authority = cleaned.split(['/', '\\', '?', '#']).next().unwrap_or("");
	let host_port = authority.rsplit_once('@').map_or(authority, |(_, host)| host);
	let host = host_port.rsplit_once(':').map_or(host_port, |(host, _)| host);
	percent_decode(host).to_lowercase()
}

/// Percent-escapes decoded; invalid UTF-8 replaced, a malformed escape kept.
fn percent_decode(s: &str) -> String {
	let bytes = s.as_bytes();
	let mut out = Vec::with_capacity(bytes.len());
	let mut i = 0;
	while i < bytes.len() {
		match bytes[i] {
			b'%' => match (bytes.get(i + 1).and_then(hex), bytes.get(i + 2).and_then(hex)) {
				(Some(hi), Some(lo)) => {
					out.push(hi << 4 | lo);
					i += 2;
				}
				_ => out.push(b'%'),
			},
			other => out.push(other),
		}
		i += 1;
	}
	String::from_utf8_lossy(&out).into_owned()
}

fn hex(byte: &u8) -> Option<u8> {
	char::from(*byte).to_digit(16).and_then(|d| u8::try_from(d).ok())
}

/// `encodeURIComponent`: everything but `A–Z a–z 0–9 - _ . ! ~ * ' ( )`
/// percent-encoded as UTF-8.
fn encode_uri_component(s: &str) -> String {
	let mut out = String::with_capacity(s.len());
	for byte in s.bytes() {
		match byte {
			b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'!' | b'~' | b'*' | b'\'' | b'(' | b')' => out.push(char::from(byte)),
			_ => push_escape(&mut out, byte),
		}
	}
	out
}

fn push_escape(out: &mut String, byte: u8) {
	const HEX: &[u8; 16] = b"0123456789ABCDEF";
	out.push('%');
	out.push(char::from(HEX[usize::from(byte >> 4)]));
	out.push(char::from(HEX[usize::from(byte & 0x0f)]));
}
