pub const TOGGLE_GROUP_ROOT: &str = "group/toggle-group flex w-fit items-center rounded-control data-[variant=outline]:shadow-xs";
/// Layered over the toggle's classes: the group's adjacency utilities.
pub const TOGGLE_GROUP_ITEM: &str = "min-w-0 flex-auto shrink-0 rounded-none shadow-none first:rounded-l-control last:rounded-r-control focus:z-10 focus-visible:z-10 data-[variant=outline]:-ml-px data-[variant=outline]:first:ml-0 data-[state=on]:z-10";

/// `Single` keeps at most one item pressed; `Multiple` allows several.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, strum::AsRefStr, strum::EnumIter, strum::IntoStaticStr)]
#[strum(serialize_all = "kebab-case")]
pub enum ToggleGroupType {
	#[default]
	Single,
	Multiple,
}
