use leptos::prelude::*;

use crate::{
	cn,
	uikit::{TOGGLE_GROUP_ITEM, TOGGLE_GROUP_ROOT, toggled},
	uikit_leptos::{
		Size, ToggleGroupType, ToggleVariant,
		primitives::{Controllable, use_controllable},
		toggle_classes,
	},
};

/// Grouped container that owns the selection, like the TS port: items carry a
/// `value` and read their pressed state from the group.
///
/// The selection is a `Vec<String>` for both types — `Single` simply never holds
/// more than one — so `value`, `default_value` and `on_value_change` keep one
/// shape whichever `type` the group has (the mirror of the TS `toArray`).
/// Pressing the selected item of a `Single` group clears it, as in TS.
#[component]
pub fn ToggleGroup(
	#[prop(optional)] r#type: ToggleGroupType,
	#[prop(optional, into)] value: MaybeProp<Vec<String>>,
	#[prop(optional)] default_value: Vec<String>,
	#[prop(optional)] on_value_change: Option<Callback<Vec<String>>>,
	#[prop(optional)] variant: ToggleVariant,
	#[prop(optional)] size: Size,
	#[prop(optional, into)] class: String,
	children: Children,
) -> impl IntoView {
	let selected = use_controllable(value, default_value, on_value_change);
	provide_context(ToggleGroupCtx {
		selected,
		kind: r#type,
		variant,
		size,
	});
	let cls = cn!(TOGGLE_GROUP_ROOT, class);
	view! {
		<div
			class=cls
			data-slot="toggle-group"
			data-variant=variant.as_ref().to_owned()
			data-size=size.as_ref().to_owned()
		>
			{children()}
		</div>
	}
}

/// A single selectable item. Reuses [`toggle_classes`] then layers the group
/// adjacency utilities.
///
/// With a `value` inside a [`ToggleGroup`] the group owns its state, and
/// `pressed`/`default_pressed`/`on_pressed_change` are ignored. Without a
/// `value` (or outside a group) it is a standalone controllable toggle.
///
/// `variant`/`size` default to the group's; an explicit one wins. (TS lets the
/// group always win, which leaves the item props dead.)
///
/// `flex-auto`, not shadcn's `flex-1`: the group is `w-fit`, and basis-0 items make
/// Chrome split it into equal columns, so a longer label overflows its cell.
#[component]
pub fn ToggleGroupItem(
	#[prop(optional, into)] value: Option<String>,
	#[prop(optional)] variant: Option<ToggleVariant>,
	#[prop(optional)] size: Option<Size>,
	#[prop(optional, into)] class: String,
	#[prop(optional, into)] disabled: Signal<bool>,
	#[prop(optional, into)] pressed: MaybeProp<bool>,
	#[prop(optional)] default_pressed: bool,
	#[prop(optional)] on_pressed_change: Option<Callback<bool>>,
	children: Children,
) -> impl IntoView {
	let own = use_controllable(pressed, default_pressed, on_pressed_change);
	let group = use_context::<ToggleGroupCtx>();
	let variant = variant.or(group.map(|g| g.variant)).unwrap_or_default();
	let size = size.or(group.map(|g| g.size)).unwrap_or_default();
	let driver = match (group, value) {
		(Some(group), Some(value)) => Driver::Group {
			group,
			value: StoredValue::new(value),
		},
		_ => Driver::Own(own),
	};
	let cls = cn!(toggle_classes(&variant, size, ""), TOGGLE_GROUP_ITEM, class);
	view! {
		<button
			type="button"
			class=cls
			data-slot="toggle-group-item"
			data-variant=variant.as_ref().to_owned()
			data-size=size.as_ref().to_owned()
			data-state=move || if driver.is_on() { "on" } else { "off" }
			aria-pressed=move || driver.is_on().to_string()
			disabled=disabled
			on:click=move |_| driver.press()
		>
			{children()}
		</button>
	}
}

#[derive(Clone, Copy)]
struct ToggleGroupCtx {
	selected: Controllable<Vec<String>>,
	kind: ToggleGroupType,
	variant: ToggleVariant,
	size: Size,
}

/// Who owns an item's pressed state.
#[derive(Clone, Copy)]
enum Driver {
	Group { group: ToggleGroupCtx, value: StoredValue<String> },
	Own(Controllable<bool>),
}

impl Driver {
	fn is_on(&self) -> bool {
		match self {
			Self::Group { group, value } => value.with_value(|value| group.selected.get().iter().any(|v| v == value)),
			Self::Own(own) => own.get(),
		}
	}

	fn press(&self) {
		match self {
			Self::Group { group, value } => {
				let next = value.with_value(|value| toggled(&group.selected.get_untracked(), value, group.kind == ToggleGroupType::Multiple, true));
				group.selected.set(next);
			}
			Self::Own(own) => own.set(!own.get_untracked()),
		}
	}
}
