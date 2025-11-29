pub mod accessibility;
pub mod navigation;
pub mod hotkey;
pub mod window;

pub use accessibility::{
    check_permissions, open_accessibility_settings, request_permissions, scan_elements,
};
pub use navigation::{activate_navigation, deactivate_navigation};
pub use hotkey::register_hotkey;
pub use window::{perform_click, show_hints};
