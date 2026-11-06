use crate::ui::uikit::core::FormControlContext;

/// The id a `FormItem` mints and the ids its label, control, description and
/// message derive from it, so `for`, `id` and `aria-describedby` line up.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct FormItemContext {
	pub id: String,
}
impl FormItemContext {
	pub(crate) fn form_item_id(&self) -> String {
		format!("{}-form-item", self.id)
	}

	pub(crate) fn form_description_id(&self) -> String {
		format!("{}-form-item-description", self.id)
	}

	pub(crate) fn form_message_id(&self) -> String {
		format!("{}-form-item-message", self.id)
	}

	/// What `FormControl` hands its control. The description always describes
	/// it; the message only while `invalid`, as only then is it rendered.
	pub(crate) fn control(&self, invalid: bool) -> FormControlContext {
		let described_by = if invalid {
			format!("{} {}", self.form_description_id(), self.form_message_id())
		} else {
			self.form_description_id()
		};
		FormControlContext {
			id: self.form_item_id(),
			described_by,
			invalid,
		}
	}
}
