pub const INPUT_OTP_CONTAINER: &str = "relative flex items-center gap-2 has-disabled:opacity-50";

pub const INPUT_OTP_INPUT: &str = "absolute inset-0 h-full w-full opacity-0 disabled:cursor-not-allowed";

pub const INPUT_OTP_GROUP: &str = "flex items-center";

pub const INPUT_OTP_SLOT: &str = "data-[active=true]:border-ring data-[active=true]:ring-ring data-[active=true]:aria-invalid:ring-accent-error \
	 aria-invalid:border-accent-error data-[active=true]:aria-invalid:border-accent-error border-input relative flex h-7 \
	 w-7 items-center justify-center border-y border-r bg-muted text-body transition-colors outline-none first:rounded-l-[var(--control-radius)] \
	 first:border-l last:rounded-r-[var(--control-radius)] data-[active=true]:z-10 data-[active=true]:ring-1";

pub const INPUT_OTP_SLOT_CARET_WRAPPER: &str = "pointer-events-none absolute inset-0 flex items-center justify-center";

pub const INPUT_OTP_SLOT_CARET: &str = "animate-caret-blink bg-ink h-4 w-px duration-1000";
