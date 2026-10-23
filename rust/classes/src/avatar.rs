use tailwind_fuse::{AsTailwindClass, TwVariant};

/// The raised, hairline-edged tile. `group/avatar` lets the fallback pick its
/// type from the size without a second prop.
pub const AVATAR: &str = "group/avatar relative flex shrink-0 overflow-hidden border border-border bg-secondary";

pub const AVATAR_IMAGE: &str = "aspect-square size-full";

/// The initial: the serif display face at the large size, where it is the whole
/// picture, and a medium sans at the two small ones.
pub const AVATAR_FALLBACK: &str = "flex size-full items-center justify-center rounded-[inherit] text-ink font-medium \
                                   group-data-[size=sm]/avatar:text-caption group-data-[size=md]/avatar:text-body-s \
                                   group-data-[size=lg]/avatar:font-display group-data-[size=lg]/avatar:text-h2 group-data-[size=lg]/avatar:font-light";

/// `Circle` is a real circle in every theme. Figma draws it at the chip radius,
/// which is 4 px in Dark and Light and so a square there (a slip, per the
/// decisions sheet); `Square` follows the chip radius.
#[derive(PartialEq, TwVariant, strum::AsRefStr, strum::EnumIter)]
#[strum(serialize_all = "kebab-case")]
pub enum AvatarShape {
	#[tw(default, class = "rounded-full")]
	Circle,
	#[tw(class = "rounded-[var(--chip-radius)]")]
	Square,
}

/// 20, 28 and 56 px.
#[derive(PartialEq, TwVariant, strum::AsRefStr, strum::EnumIter)]
#[strum(serialize_all = "kebab-case")]
pub enum AvatarSize {
	#[tw(class = "size-5")]
	Sm,
	#[tw(default, class = "size-7")]
	Md,
	#[tw(class = "size-14")]
	Lg,
}
