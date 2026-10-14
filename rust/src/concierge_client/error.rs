use tonic_concierge::{Code, Status};

/// What a failed call to concierge means to the relying party. Each arm is a different
/// reaction: sign the user out, retry later, show the operator what was refused, or report
/// a bug.
#[derive(Debug, thiserror::Error)]
pub enum ConciergeError {
	/// UNAUTHENTICATED or PERMISSION_DENIED: a wrong client secret, a code already
	/// redeemed, a revoked refresh family, an access token that expired. Ends the session.
	#[error("concierge refused the call: {code:?}")]
	Refused { code: Code },
	/// INVALID_ARGUMENT or FAILED_PRECONDITION: concierge read the request and said no on
	/// its merits — a catalog outside the namespace, an older catalog version. The message
	/// is concierge's own and says which rule.
	#[error("concierge rejected the request ({code:?}): {message}")]
	Rejected { code: Code, message: String },
	/// Not reachable, or no answer within the configured timeout. Worth a retry.
	#[error("concierge is unavailable: {0}")]
	Unavailable(String),
	/// concierge answered, but with something this client cannot use (a user id that is
	/// not a UUID, an expiry out of range).
	#[error("concierge answered something unusable: {0}")]
	InvalidResponse(String),
	/// Built wrong on this side: an endpoint that is not a URI, a credential that cannot
	/// travel as a header.
	#[error("concierge client misconfigured: {0}")]
	Config(String),
	/// The OS refused randomness for a PKCE verifier or a state.
	#[error("no randomness for {what}: {source}")]
	Randomness {
		what: &'static str,
		#[source]
		source: getrandom::Error,
	},
	/// Any other status: a bug on one side or the other.
	#[error("concierge failed ({code:?}): {message}")]
	Failed { code: Code, message: String },
}

impl From<Status> for ConciergeError {
	fn from(status: Status) -> Self {
		let code = status.code();
		let message = status.message().to_owned();
		match code {
			Code::Unauthenticated | Code::PermissionDenied => Self::Refused { code },
			Code::InvalidArgument | Code::FailedPrecondition => Self::Rejected { code, message },
			Code::Unavailable | Code::DeadlineExceeded | Code::Cancelled => Self::Unavailable(message),
			Code::Ok
			| Code::Unknown
			| Code::NotFound
			| Code::AlreadyExists
			| Code::ResourceExhausted
			| Code::Aborted
			| Code::OutOfRange
			| Code::Unimplemented
			| Code::Internal
			| Code::DataLoss => Self::Failed { code, message },
		}
	}
}

impl ConciergeError {
	/// Whether retrying the same call later may succeed.
	pub fn is_transient(&self) -> bool {
		matches!(self, Self::Unavailable(_))
	}
}
