/// One child of an icon's 24×24 `<svg>`. Attribute values are kept as written so
/// every port renders them byte for byte.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Shape {
	Path {
		d: &'static str,
	},
	Circle {
		cx: &'static str,
		cy: &'static str,
		r: &'static str,
	},
	Rect {
		x: &'static str,
		y: &'static str,
		width: &'static str,
		height: &'static str,
		rx: &'static str,
		ry: Option<&'static str>,
	},
}

/// The kit's glyphs (lucide geometry), inlined so it keeps its zero-icon-dep
/// footprint. Only the children: each site keeps its own `<svg>` frame.
#[derive(Clone, Copy, Debug, Eq, PartialEq, strum::AsRefStr, strum::EnumIter, strum::IntoStaticStr)]
#[strum(serialize_all = "kebab-case")]
pub enum Icon {
	Alert,
	ArrowLeft,
	ArrowUp,
	Bell,
	Calendar,
	Check,
	ChevronDown,
	ChevronLeft,
	ChevronRight,
	/// The `x` drawn as two strokes.
	Close,
	Copy,
	Dot,
	Ellipsis,
	File,
	Image,
	Info,
	Layers,
	Lock,
	Minus,
	Plus,
	Refresh,
	Search,
	SlidersHorizontal,
	Sparkles,
	Spinner,
	Stop,
	TriangleAlert,
	X,
}

impl Icon {
	pub fn shapes(self) -> &'static [Shape] {
		use Shape::{Circle, Path, Rect};
		const RING: Shape = Circle { cx: "12", cy: "12", r: "10" };
		match self {
			Icon::Alert => &[RING, Path { d: "M12 8v4M12 16h.01" }],
			Icon::ArrowLeft => &[Path { d: "m12 19-7-7 7-7" }, Path { d: "M19 12H5" }],
			Icon::ArrowUp => &[Path { d: "m5 12 7-7 7 7M12 19V5" }],
			Icon::Bell => &[
				Path {
					d: "M6 8a6 6 0 0 1 12 0c0 7 3 9 3 9H3s3-2 3-9",
				},
				Path {
					d: "M10.3 21a1.94 1.94 0 0 0 3.4 0",
				},
			],
			Icon::Calendar => &[
				Path { d: "M8 2v4" },
				Path { d: "M16 2v4" },
				Path { d: "M3 10h18" },
				Path {
					d: "M5 4h14a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V6a2 2 0 0 1 2-2z",
				},
			],
			Icon::Check => &[Path { d: "M20 6 9 17l-5-5" }],
			Icon::ChevronDown => &[Path { d: "m6 9 6 6 6-6" }],
			Icon::ChevronLeft => &[Path { d: "m15 18-6-6 6-6" }],
			Icon::ChevronRight => &[Path { d: "m9 18 6-6-6-6" }],
			Icon::Close => &[Path { d: "M18 6 6 18" }, Path { d: "m6 6 12 12" }],
			Icon::Copy => &[
				Rect {
					x: "8",
					y: "8",
					width: "14",
					height: "14",
					rx: "2",
					ry: None,
				},
				Path {
					d: "M4 16c-1.1 0-2-.9-2-2V4c0-1.1.9-2 2-2h10c1.1 0 2 .9 2 2",
				},
			],
			Icon::Dot => &[RING],
			Icon::Ellipsis => &[Circle { cx: "12", cy: "12", r: "1" }, Circle { cx: "19", cy: "12", r: "1" }, Circle { cx: "5", cy: "12", r: "1" }],
			Icon::File => &[
				Path {
					d: "M15 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7Z",
				},
				Path { d: "M14 2v4a2 2 0 0 0 2 2h4" },
			],
			Icon::Image => &[
				Rect {
					x: "3",
					y: "3",
					width: "18",
					height: "18",
					rx: "2",
					ry: None,
				},
				Circle { cx: "9", cy: "9", r: "2" },
				Path {
					d: "m21 15-3.086-3.086a2 2 0 0 0-2.828 0L6 21",
				},
			],
			Icon::Info => &[RING, Path { d: "M12 16v-4" }, Path { d: "M12 8h.01" }],
			Icon::Layers => &[
				Path {
					d: "m12.83 2.18a2 2 0 0 0-1.66 0L2.6 6.08a1 1 0 0 0 0 1.83l8.58 3.91a2 2 0 0 0 1.66 0l8.58-3.9a1 1 0 0 0 0-1.83Z",
				},
				Path {
					d: "m22 17.65-9.17 4.16a2 2 0 0 1-1.66 0L2 17.65M22 12.65l-9.17 4.16a2 2 0 0 1-1.66 0L2 12.65",
				},
			],
			Icon::Lock => &[
				Rect {
					x: "3",
					y: "11",
					width: "18",
					height: "11",
					rx: "2",
					ry: Some("2"),
				},
				Path { d: "M7 11V7a5 5 0 0 1 10 0v4" },
			],
			Icon::Minus => &[Path { d: "M5 12h14" }],
			Icon::Plus => &[Path { d: "M5 12h14M12 5v14" }],
			Icon::Refresh => &[
				Path {
					d: "M3 12a9 9 0 0 1 9-9 9.75 9.75 0 0 1 6.74 2.74L21 8",
				},
				Path { d: "M21 3v5h-5" },
				Path {
					d: "M21 12a9 9 0 0 1-9 9 9.75 9.75 0 0 1-6.74-2.74L3 16",
				},
				Path { d: "M8 16H3v5" },
			],
			Icon::Search => &[Circle { cx: "11", cy: "11", r: "8" }, Path { d: "m21 21-4.3-4.3" }],
			Icon::SlidersHorizontal => &[
				Path { d: "M10 5H3" },
				Path { d: "M12 19H3" },
				Path { d: "M14 3v4" },
				Path { d: "M16 17v4" },
				Path { d: "M21 12h-9" },
				Path { d: "M21 19h-5" },
				Path { d: "M21 5h-7" },
				Path { d: "M8 10v4" },
				Path { d: "M8 12H3" },
			],
			Icon::Sparkles => &[Path {
				d: "M9.9 4.4 11 8l3.6 1.1L11 10.2 9.9 13.8 8.8 10.2 5.2 9.1 8.8 8ZM18 14l.7 2.3L21 17l-2.3.7L18 20l-.7-2.3L15 17l2.3-.7Z",
			}],
			Icon::Spinner => &[Path { d: "M21 12a9 9 0 1 1-6.219-8.56" }],
			Icon::Stop => &[Rect {
				x: "6",
				y: "6",
				width: "12",
				height: "12",
				rx: "2",
				ry: None,
			}],
			Icon::TriangleAlert => &[
				Path {
					d: "m21.73 18-8-14a2 2 0 0 0-3.48 0l-8 14A2 2 0 0 0 4 21h16a2 2 0 0 0 1.73-3",
				},
				Path { d: "M12 9v4" },
				Path { d: "M12 17h.01" },
			],
			Icon::X => &[Path { d: "M18 6 6 18M6 6l12 12" }],
		}
	}
}
