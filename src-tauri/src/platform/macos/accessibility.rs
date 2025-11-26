use cocoa::base::{id, nil};
use cocoa::foundation::NSString;
use core_foundation::*;

// Verifica si tenemos permisos de accesibilidad
pub fn has_accessibility_permissions() -> bool {
    unsafe {
        let trusted = AXIsProcessTrustedWithOptions(nil);
        trusted != 0
    }
}

// Solicita permisos abriendo Preferencias del sistema
pub fn request_permissions() -> Result<()> {
    unsafe {
        let options = AXIsProcessTrustedWithOptions(options);
    }
    Ok(())
}

// Obtiene la ventana activa del sistema 
pub fn get_active_window() -> Result<id> {
    unsafe {
        let sustem_wide = AXUIElementCreateSystemWide();
        let mut focused_app: id = nil;

        AXUIElementCopyAttributeValue(
            sustem_wide,
            kAXFocusedApplicationAttribute,
            &mut focused_app as *mut _ as *mut _,
        );

        Ok(focused_app)
    }
}

// Recorre el arbol de accesibilidad recursivamente
pub fn traverse_accessibility_tree(element:id) -> Vec<id> {
    // Implementacion recursiva para obtener tods los elementos 
}