use crate::error::AppError;
use crate::models::UIElement;
use crate::platform::macos::accessibility::{
    self, get_active_window, get_element_rect, get_element_role, get_element_title,
    is_clickable_role, traverse_accessibility_tree, AXUIElementRef,
};
use crate::Result;
use core_foundation::base::CFRelease;
use std::ffi::c_void;

const DEFAULT_MAX_DEPTH: usize = 10;
const DEFAULT_MAX_ELEMENTS: usize = 500;

pub struct AccessibilityService {
    max_depth: usize,
    max_elements: usize,
}

impl AccessibilityService {
    pub fn new() -> Self {
        tracing::debug!("Initializing AccessibilityService");
        Self {
            max_depth: DEFAULT_MAX_DEPTH,
            max_elements: DEFAULT_MAX_ELEMENTS,
        }
    }

    pub fn with_config(max_depth: usize, max_elements: usize) -> Self {
        tracing::debug!(
            "Initializing AccessibilityService (max_depth={}, max_elements={})",
            max_depth,
            max_elements
        );
        Self {
            max_depth,
            max_elements,
        }
    }

    pub fn check_permissions(&self) -> bool {
        tracing::trace!("Checking permissions");
        let has_perms = accessibility::has_accessibility_permissions();
        if !has_perms {
            tracing::warn!("Accessibility permissions NOT granted");
        }
        has_perms
    }

    pub fn request_permissions(&self) -> Result<()> {
        tracing::info!("Requesting permissions");
        accessibility::request_permissions()
    }

    pub fn get_focused_application_pid(&self) -> Result<Option<i32>> {
        tracing::trace!("Getting focused application PID");
        accessibility::get_focused_application()
    }

    pub fn ensure_permissions(&self) -> Result<()> {
        if self.check_permissions() {
            Ok(())
        } else {
            tracing::error!("Accessibility permissions denied");
            Err(AppError::Accessibility(
        "Accessibility permissions not granted. Enable in System Preferences > Privacy & Security > Accessibility".to_string(),
      ))
        }
    }

    fn map_ax_element(&self, element: AXUIElementRef, index: usize) -> Result<UIElement> {
        let role = get_element_role(element)?;
        let title = get_element_title(element);
        let rect = get_element_rect(element)?;

        let id = format!("ax-{}-{}", index, role.to_lowercase().replace("ax", ""));

        tracing::trace!("Mapped: {} role={}", id, role);

        Ok(UIElement::new(
            id,
            role,
            title,
            rect.x,
            rect.y,
            rect.width,
            rect.height,
        ))
    }

    /// Libera un AXUIElementRef de forma segura
    fn release_element(&self, element: AXUIElementRef) {
        if !element.is_null() {
            // SAFETY: CFRelease es segura con un CFTypeRef válido no nulo
            unsafe { CFRelease(element as *const c_void) };
        }
    }

    pub async fn scan_clickable_elements(&self) -> Result<Vec<UIElement>> {
        tracing::info!("Scanning clickable elements");

        self.ensure_permissions()?;

        let active_window = get_active_window()?;

        let ax_elements =
            traverse_accessibility_tree(active_window, self.max_depth, self.max_elements);

        tracing::debug!("Found {} AX elements", ax_elements.len());

        let mut ui_elements = Vec::with_capacity(ax_elements.len());

        for (index, ax_element) in ax_elements.iter().enumerate() {
            match self.map_ax_element(*ax_element, index) {
                Ok(ui_element) => {
                    if is_clickable_role(&ui_element.role)
                        && ui_element.width > 1.0
                        && ui_element.height > 1.0
                    {
                        ui_elements.push(ui_element);
                    }
                }
                Err(err) => {
                    tracing::debug!("Skipping element {}: {}", index, err);
                }
            }

            // Liberar el AXUIElementRef después de procesarlo
            self.release_element(*ax_element);
        }

        // Liberar la ventana activa
        self.release_element(active_window);

        tracing::info!("Found {} clickable elements", ui_elements.len());

        Ok(ui_elements)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_service_creation() {
        let service = AccessibilityService::new();
        assert_eq!(service.max_depth, DEFAULT_MAX_DEPTH);
        assert_eq!(service.max_elements, DEFAULT_MAX_ELEMENTS);
    }

    #[test]
    fn test_service_with_config() {
        let service = AccessibilityService::with_config(5, 100);
        assert_eq!(service.max_depth, 5);
        assert_eq!(service.max_elements, 100);
    }
}
