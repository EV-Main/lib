/// The element a `SectionLabel` renders as. The TS `as` takes any element
/// type; neither port has a dynamic tag, so this lists the ones a label
/// plausibly is.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum SectionLabelAs {
	#[default]
	P,
	Span,
	Div,
	H2,
	H3,
	H4,
}
