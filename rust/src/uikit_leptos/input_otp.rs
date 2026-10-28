use leptos::{context::Provider, prelude::*};

use crate::{
	cn,
	uikit::InputOtp,
	uikit_leptos::{
		INPUT_OTP_CONTAINER, INPUT_OTP_GROUP, INPUT_OTP_INPUT, INPUT_OTP_SLOT, INPUT_OTP_SLOT_CARET, INPUT_OTP_SLOT_CARET_WRAPPER,
		primitives::{Controllable, use_controllable},
	},
};

#[component]
pub fn InputOTP(
	max_length: usize,
	#[prop(optional, into)] value: MaybeProp<String>,
	#[prop(optional, into)] default_value: String,
	#[prop(optional)] on_change: Option<Callback<String>>,
	#[prop(optional, into)] class: String,
	#[prop(optional, into)] container_class: String,
	#[prop(optional, into)] disabled: Signal<bool>,
	children: Children,
) -> impl IntoView {
	let value = use_controllable(value, default_value, on_change);
	let focused = RwSignal::new(false);
	let otp = InputOtp { max_length };
	let container_cls = cn!(INPUT_OTP_CONTAINER, container_class);
	let input_cls = cn!(INPUT_OTP_INPUT, class);
	view! {
		<div class=container_cls data-slot="input-otp">
			<Provider value=InputOtpContext {
				value,
				focused,
				otp,
			}>{children()}</Provider>
			<input
				inputmode="numeric"
				autocomplete="one-time-code"
				maxlength=max_length.to_string()
				value=value.get_untracked()
				prop:value=move || value.get()
				disabled=disabled
				class=input_cls
				on:input=move |e| value.set(otp.accept(&event_target_value(&e)))
				on:focus=move |_| focused.set(true)
				on:blur=move |_| focused.set(false)
			/>
		</div>
	}
}

#[component]
pub fn InputOTPGroup(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
	let cls = cn!(INPUT_OTP_GROUP, class);
	view! {
		<div class=cls data-slot="input-otp-group">
			{children()}
		</div>
	}
}

#[component]
pub fn InputOTPSlot(index: usize, #[prop(optional, into)] class: String) -> impl IntoView {
	let ctx = expect_context::<InputOtpContext>();
	let slot = Memo::new(move |_| ctx.otp.slot(&ctx.value.get(), ctx.focused.get(), index));
	let cls = cn!(INPUT_OTP_SLOT, class);
	view! {
		<div class=cls data-slot="input-otp-slot" data-active=move || slot.get().active.to_string()>
			{move || slot.get().char.map(|c| c.to_string())}
			{move || {
				slot
					.get()
					.caret
					.then(|| {
						view! {
							<div class=INPUT_OTP_SLOT_CARET_WRAPPER>
								<div class=INPUT_OTP_SLOT_CARET></div>
							</div>
						}
					})
			}}
		</div>
	}
}

#[component]
pub fn InputOTPSeparator() -> impl IntoView {
	view! {
		<div data-slot="input-otp-separator" role="separator">
			<svg
				xmlns="http://www.w3.org/2000/svg"
				viewBox="0 0 24 24"
				fill="none"
				stroke="currentColor"
				stroke-width="2"
				stroke-linecap="round"
				stroke-linejoin="round"
				aria-hidden="true"
			>
				<path d="M5 12h14"></path>
			</svg>
		</div>
	}
}

#[derive(Clone, Copy)]
struct InputOtpContext {
	value: Controllable<String>,
	focused: RwSignal<bool>,
	otp: InputOtp,
}
