use cocoa::base::{id, nil};

use crate::Result;

pub fn has_accessibility_permissions() -> bool {
    true
}

pub fn request_permissions() -> Result<()> {
    Ok(())
}

pub fn get_active_window() -> Result<id> {
    Ok(nil)
}

pub fn traverse_accessibility_tree(_element: id) -> Vec<id> {
    Vec::new()
}
