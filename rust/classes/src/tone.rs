use tailwind_fuse::{AsTailwindClass, TwVariant};

/// The kit's one severity vocabulary (docs/spec/variants.md): the accent rungs
/// a status needs, the valence and the action role. A tone sets `--tone-fill`,
/// `--tone-edge` and `--tone-ink`; a component reads the parts it wears. Every
/// tone sets all three, so a tone nested in another inherits none of it.
#[derive(Debug, PartialEq, TwVariant, strum::AsRefStr, strum::EnumIter)]
#[strum(serialize_all = "kebab-case")]
pub enum Tone {
	#[tw(default, class = "[--tone-fill:var(--tint-neutral-fill)] [--tone-edge:var(--tint-neutral-edge)] [--tone-ink:var(--ink)]")]
	Neutral,
	#[tw(class = "[--tone-fill:var(--tint-primary-fill)] [--tone-edge:var(--tint-primary-edge)] [--tone-ink:var(--primary-ink)]")]
	Primary,
	#[tw(class = "[--tone-fill:var(--tint-positive-fill)] [--tone-edge:var(--tint-positive-edge)] [--tone-ink:var(--positive-ink)]")]
	Positive,
	#[tw(class = "[--tone-fill:var(--tint-info-fill)] [--tone-edge:var(--tint-info-edge)] [--tone-ink:var(--accent-info-ink)]")]
	Info,
	#[tw(class = "[--tone-fill:var(--tint-warn-fill)] [--tone-edge:var(--tint-warn-edge)] [--tone-ink:var(--accent-warn)]")]
	Warn,
	#[tw(class = "[--tone-fill:var(--tint-error-fill)] [--tone-edge:var(--tint-error-edge)] [--tone-ink:var(--accent-error)]")]
	Error,
}
