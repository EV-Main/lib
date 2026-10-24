use tailwind_fuse::{AsTailwindClass, TwVariant};

pub const CHAT_BUBBLE_BASE: &str = "min-w-0 break-words whitespace-pre-wrap text-body text-ink";

/// `assistant` has no box, so a long answer reads as a page.
#[derive(Debug, PartialEq, TwVariant, strum::AsRefStr, strum::EnumIter)]
#[strum(serialize_all = "kebab-case")]
pub enum ChatBubbleVariant {
	#[tw(class = "max-w-105 rounded-lg border border-border bg-secondary px-3.5 py-2.5")]
	User,
	#[tw(default, class = "py-0.5")]
	Assistant,
	#[tw(class = "max-w-105 rounded-lg border border-border-subtle bg-muted px-3.5 py-2.5")]
	Muted,
}

pub const CHAT_MESSAGE_BASE: &str = "flex gap-3";

pub const CHAT_MESSAGE_USER: &str = "justify-end";

pub const CHAT_MESSAGE_AVATAR: &str = "shrink-0";

/// The default sparkle tile of an assistant turn.
pub const CHAT_MESSAGE_AVATAR_TILE: &str = "flex size-7 items-center justify-center rounded-(--control-radius) border border-border bg-secondary text-ink-mid";

pub const CHAT_MESSAGE_BODY: &str = "flex min-w-0 flex-col gap-1.5";

pub const CHAT_MESSAGE_BODY_USER: &str = "max-w-105 items-end";

pub const CHAT_MESSAGE_BODY_ASSISTANT: &str = "flex-1";

pub const CHAT_MESSAGE_HEADER: &str = "flex items-baseline gap-2";

pub const CHAT_MESSAGE_NAME: &str = "text-body-s font-medium text-ink";

pub const CHAT_MESSAGE_TIME: &str = "text-caption text-ink-soft";

pub const CHAT_MARKER_SEPARATOR: &str = "flex items-center gap-3";

/// Each of the two hairlines either side of a separator's label.
pub const CHAT_MARKER_RULE: &str = "h-px flex-1 bg-border";

pub const CHAT_MARKER_SEPARATOR_LABEL: &str = "text-caption text-ink-soft";

pub const CHAT_MARKER_STATUS: &str = "flex items-center gap-2 text-body-s text-ink-soft";

pub const CHAT_MARKER_DOT: &str = "size-1.5 rounded-full";

/// The dot of a status marker: it pulses only while the work runs.
#[derive(Debug, PartialEq, TwVariant, strum::AsRefStr, strum::EnumIter)]
#[strum(serialize_all = "kebab-case")]
pub enum ChatStatusState {
	#[tw(class = "bg-primary-ink motion-safe:animate-pulse")]
	Running,
	#[tw(default, class = "bg-ink-soft")]
	Done,
}

pub const CHAT_MARKER_TOOL: &str = "flex items-center gap-2.5 rounded-(--control-radius) border bg-card px-2.5 py-2";

pub const CHAT_MARKER_TOOL_BORDER: &str = "border-border";

pub const CHAT_MARKER_TOOL_ERROR_BORDER: &str = "border-chip-error-border";

/// The tool glyph, and the same box for the check of a finished call.
pub const CHAT_MARKER_TOOL_ICON: &str = "size-4 shrink-0 text-ink-mid";

pub const CHAT_MARKER_TOOL_CHECK: &str = "size-4 shrink-0 text-ink-soft";

pub const CHAT_MARKER_TOOL_BODY: &str = "flex min-w-0 flex-1 flex-col gap-px";

pub const CHAT_MARKER_TOOL_TITLE: &str = "truncate text-body-s font-medium text-ink";

pub const CHAT_MARKER_TOOL_DETAIL: &str = "truncate text-body-s tabular-nums text-ink-soft";

pub const CHAT_MARKER_TOOL_META: &str = "shrink-0 text-caption";

/// A tool call's state; the class colours its right-hand note.
#[derive(Debug, PartialEq, TwVariant, strum::AsRefStr, strum::EnumIter)]
#[strum(serialize_all = "kebab-case")]
pub enum ChatToolState {
	#[tw(class = "text-primary-ink")]
	Running,
	#[tw(default, class = "text-ink-soft")]
	Done,
	#[tw(class = "text-accent-error-ink")]
	Error,
}

pub const CHAT_ATTACHMENT_BASE: &str = "flex w-60 max-w-full flex-col overflow-hidden rounded-(--control-radius) border border-border bg-card";

pub const CHAT_ATTACHMENT_ROW: &str = "flex items-center gap-2.5 p-2";

pub const CHAT_ATTACHMENT_ICON: &str = "flex size-9 shrink-0 items-center justify-center rounded-(--control-radius) border border-border-subtle bg-muted text-ink-mid";

pub const CHAT_ATTACHMENT_BODY: &str = "flex min-w-0 flex-1 flex-col gap-px";

pub const CHAT_ATTACHMENT_NAME: &str = "truncate text-body-s font-medium text-ink";

pub const CHAT_ATTACHMENT_META: &str = "truncate text-caption text-ink-soft";

pub const CHAT_ATTACHMENT_PROGRESS: &str = "h-0.5 bg-border-subtle";

pub const CHAT_ATTACHMENT_PROGRESS_FILL: &str = "h-full bg-primary";
