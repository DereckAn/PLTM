pub mod accessibility;
pub mod hotkey;
pub mod window;

pub use accessibility::{check_permissions, request_permissions, scan_elements};
pub use hotkey::register_hotkey;
pub use window::{perform_click, show_hints};