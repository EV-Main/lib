//! The kit's fixed English: strings a component renders that no Labels struct
//! carries. Namespaced, as several share a name with a class const.

pub const CLOSE: &str = "Close";
pub const LOADING: &str = "Loading";
pub const MORE_INFORMATION: &str = "More information";

pub const BREADCRUMB: &str = "breadcrumb";
pub const BREADCRUMB_MORE: &str = "More";

pub const PAGINATION: &str = "pagination";
pub const PAGINATION_PREVIOUS: &str = "Previous";
pub const PAGINATION_PREVIOUS_PAGE: &str = "Go to previous page";
pub const PAGINATION_NEXT: &str = "Next";
pub const PAGINATION_NEXT_PAGE: &str = "Go to next page";
pub const PAGINATION_MORE: &str = "More pages";

pub const CAROUSEL_PREVIOUS: &str = "Previous slide";
pub const CAROUSEL_NEXT: &str = "Next slide";

pub const SIDEBAR_TOGGLE: &str = "Toggle Sidebar";

pub const CALENDAR_PREVIOUS_MONTH: &str = "Previous month";
pub const CALENDAR_NEXT_MONTH: &str = "Next month";
pub const MONTHS: [&str; 12] = [
	"January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December",
];
/// Monday first.
pub const WEEKDAYS: [&str; 7] = ["Mo", "Tu", "We", "Th", "Fr", "Sa", "Su"];

pub const MODEL_CHIP: &str = "Model";
pub const CHAT_COMPOSER_PLACEHOLDER: &str = "Ask the assistant…";
pub const CHAT_TYPING: &str = "Assistant is thinking";

pub const ASIDE_RAIL: &str = "Panels";
pub const WORKSPACE_TAB_CLOSE: &str = "Close tab";

pub const HEADER_BALANCE: &str = "Balance:";
pub const HEADER_SEARCH: &str = "Search or jump to…";
/// The `HeaderSearch` hint until the platform is known: the server render and
/// the first client render.
pub const CTRL_K: &str = "Ctrl K";
pub const COMMAND_K: &str = "⌘K";
pub const NOTIFICATION_BELL_TRIGGER: &str = "Notifications";
pub const NOTIFICATION_BELL_EMPTY: &str = "No notifications";

pub const FOOTER_OFFICES: &str = "Offices";
pub const FOOTER_NEWSLETTER: &str = "Newsletter";

pub const STATUS_HOME: &str = "Back to home";
pub const NOT_FOUND_EYEBROW: &str = "Page not found";
pub const NOT_FOUND_HEADLINE_LEAD: &str = "You've reached ";
pub const NOT_FOUND_HEADLINE_ACCENT: &str = "open water";
pub const NOT_FOUND_SUBTEXT: &str = "The page you're looking for has drifted off our coastline — moved, renamed, or never charted. Let's get you back to shore.";
pub const NOT_FOUND_CONTACT: &str = "Contact the team";
pub const FORBIDDEN_EYEBROW: &str = "Access forbidden";
pub const FORBIDDEN_HEADLINE_LEAD: &str = "This harbour is ";
pub const FORBIDDEN_HEADLINE_ACCENT: &str = "private";
pub const FORBIDDEN_SUBTEXT: &str = "You don't have the credentials to view this page. If you believe you should, our team can open the right doors.";
pub const FORBIDDEN_CONTACT: &str = "Request access";
pub const SERVER_ERROR_EYEBROW: &str = "Server error";
pub const SERVER_ERROR_HEADLINE_LEAD: &str = "Our systems are ";
pub const SERVER_ERROR_HEADLINE_ACCENT: &str = "recalibrating";
pub const SERVER_ERROR_SUBTEXT: &str = "Something broke on our end — not yours. We've been alerted and are restoring service. Please try again in a moment.";
pub const SERVER_ERROR_RETRY: &str = "Try again";
