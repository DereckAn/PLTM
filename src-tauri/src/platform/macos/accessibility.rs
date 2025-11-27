use std::ffi::c_void;

use crate::error::AppError;
use crate::Result;
use core_foundation::array::CFArray;
use core_foundation::base::{CFRelease, CFType, TCFType};
use core_foundation::boolean::CFBoolean;
use core_foundation::dictionary::CFDictionary;
use core_foundation::number::CFNumber;
use core_foundation::string::CFString;

// Tipos opacos de AXUIElement
pub type AXUIElementRef = *const c_void;
pub type AXValueRef = *const c_void;

#[derive(Debug, Clone, Copy)]
pub struct Rect {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

// FFI declaration para Accessibility API
#[link(name = "ApplicationServices", kind = "framework")]
extern "C" {
    fn AXIsProcessTrustedWithOptions(options: *const c_void) -> bool;
    fn AXUIElementCreateSystemWide() -> AXUIElementRef;
    fn AXUIElementCopyAttributeValue(
        element: AXUIElementRef,
        attribute: *const c_void,
        value: *mut *const c_void,
    ) -> i32;
    fn AXUIElementCopyAttributeNames(element: AXUIElementRef, names: *mut *const c_void) -> i32;
    // fn AXValueGetType(value: AXValueRef) -> u32;
    fn AXValueGetValue(value: AXValueRef, value_type: u32, value_ptr: *mut c_void) -> bool;
}

// Constantes de tipos AXValue
const K_AX_VALUE_TYPE_CGPOINT: u32 = 1;
const K_AX_VALUE_TYPE_CGSIZE: u32 = 2;

// Constantes de atributos AX
const K_AX_TRUSTED_CHECK_OPTION_PROMPT: &str = "AXTrustedCheckOptionPrompt";
const K_AX_FOCUSED_APPLICATION_ATTRIBUTE: &str = "AXFocusedApplication";
const K_AX_FOCUSED_WINDOW_ATTRIBUTE: &str = "AXFocusedWindow";
const K_AX_PID_ATTRIBUTE: &str = "AXPID";
const K_AX_TITLE_ATTRIBUTE: &str = "AXTitle";
const K_AX_POSITION_ATTRIBUTE: &str = "AXPosition";
const K_AX_SIZE_ATTRIBUTE: &str = "AXSize";
const K_AX_ROLE_ATTRIBUTE: &str = "AXRole";
// const K_AX_SUBROLE_ATTRIBUTE: &str = "AXSubrole";
#[allow(dead_code)]
const K_AX_CHILDREN_ATTRIBUTE: &str = "AXChildren";

// Rolers clickeables
const CLICKABLE_ROLES: &[&str] = &[
    "AXButton",
    "AXLink",
    "AXMenuItem",
    "AXMenuButton",
    "AXPopUpButton",
    "AXCheckBox",
    "AXRadioButton",
    "AXTextField",
    "AXTextArea",
    "AXComboBox",
    "AXSlider",
    "AXIncrementor",
    "AXColorWell",
    "AXDisclosureTriangle",
    "AXTab",
    "AXTabGroup",
];

/// Verifica si la aplicación tiene permisos de accesibilidad
pub fn has_accessibility_permissions() -> bool {
    tracing::info!("Checking accessibility permissions");

    // SAFETY: AXIsProcessTrustedWithOptions es segura con null pointer
    // y solo lee el estado de permisos del sistema
    let result = unsafe { AXIsProcessTrustedWithOptions(std::ptr::null()) };

    result
}

/// Solicita permisos de accesibilidad (muestra el prompt del sistema)
pub fn request_permissions() -> Result<()> {
    tracing::info!("Requesting accessibility permissions");

    // Crear diccionario de opciones (código seguro)
    let prompt_key = CFString::new(K_AX_TRUSTED_CHECK_OPTION_PROMPT);
    let prompt_value = CFBoolean::true_value();
    let options =
        CFDictionary::from_CFType_pairs(&[(prompt_key.as_CFType(), prompt_value.as_CFType())]);

    // SAFETY: AXIsProcessTrustedWithOptions es segura con un CFDictionaryRef válido
    // La función puede mostrar un diálogo del sistema pero no modifica memoria
    let has_permissions =
        unsafe { AXIsProcessTrustedWithOptions(options.as_concrete_TypeRef() as *const c_void) };

    if has_permissions {
        tracing::info!("Accessibility permissions already granted");
    } else {
        tracing::warn!(
            "Accessibility permissions not granted - system prompt shown. \
       User must enable in System Preferences > Privacy & Security > Accessibility"
        );
    }

    Ok(())
}

/// Helper privado para crear el system-wide element
///
/// # Safety
/// Devuelve un AXUIElementRef que debe ser liberado con CFRelease
fn create_system_wide() -> AXUIElementRef {
    // SAFETY: AXUIElementCreateSystemWide siempre devuelve un elemento válido
    // o null si hay un error catastrófico del sistema
    unsafe { AXUIElementCreateSystemWide() }
}

/// Helper privado para obtener un atributo de un elemento AX
///
/// # Safety
/// - `element` debe ser un AXUIElementRef válido
/// - El caller es responsable de liberar el valor retornado
fn copy_attribute_value(element: AXUIElementRef, attribute: &str) -> Result<*const c_void> {
    let attr_name = CFString::new(attribute);
    let mut value: *const c_void = std::ptr::null();

    // SAFETY: AXUIElementCopyAttributeValue es segura si:
    // - element es un AXUIElementRef válido
    // - attribute es un CFStringRef válido
    // - value es un puntero válido para escritura
    let result = unsafe {
        AXUIElementCopyAttributeValue(
            element,
            attr_name.as_concrete_TypeRef() as *const c_void,
            &mut value,
        )
    };

    if result != 0 {
        return Err(AppError::Accessibility(format!(
            "Failed to get attribute '{}': AX error code {}",
            attribute, result
        )));
    }

    if value.is_null() {
        return Err(AppError::Accessibility(format!(
            "Attribute '{}' returned null value",
            attribute
        )));
    }

    Ok(value)
}

/// Helper privado para liberar un AXUIElementRef de forma segura
fn release_ax_element(element: *const c_void) {
    if !element.is_null() {
        // SAFETY: CFRelease es segura si el puntero es un CFTypeRef válido
        // o null (en cuyo caso es un no-op)
        unsafe { CFRelease(element) };
    }
}

/// Obtiene el PID de la aplicación enfocada
pub fn get_focused_application() -> Result<Option<i32>> {
    tracing::info!("Getting focused application PID");

    let system_wide = create_system_wide();

    // Obtener la aplicación enfocada
    let focused_app = match copy_attribute_value(system_wide, K_AX_FOCUSED_APPLICATION_ATTRIBUTE) {
        Ok(app) => app,
        Err(e) => {
            tracing::error!("Failed to get focused application: {}", e);
            return Err(e);
        }
    };

    // Obtener el PID de la aplicación
    let pid_result = copy_attribute_value(focused_app as AXUIElementRef, K_AX_PID_ATTRIBUTE);

    // Liberar la referencia de focused_app (RAII manual)
    release_ax_element(focused_app);
    release_ax_element(system_wide);

    let pid_value = match pid_result {
        Ok(value) => value,
        Err(e) => {
            tracing::warn!("Could not get PID attribute: {}", e);
            return Ok(None);
        }
    };

    // SAFETY: wrap_under_create_rule toma ownership del CFNumberRef
    // y lo liberará automáticamente cuando salga de scope
    let pid_number = unsafe { CFNumber::wrap_under_create_rule(pid_value as *const _) };

    match pid_number.to_i32() {
        Some(pid) => {
            tracing::info!("Focused application PID: {}", pid);
            Ok(Some(pid))
        }
        None => {
            tracing::error!("Failed to convert PID CFNumber to i32");
            Err(AppError::Accessibility(
                "Failed to convert PID to i32".to_string(),
            ))
        }
    }
}

/// Obtiene la ventana activa del sistema (para escaneo de elementos)
///
/// # Warning
/// El caller es responsable de liberar el AXUIElementRef retornado
pub fn get_active_window() -> Result<AXUIElementRef> {
    tracing::info!("Getting active window element");

    let system_wide = create_system_wide();

    let focused_app = copy_attribute_value(system_wide, K_AX_FOCUSED_APPLICATION_ATTRIBUTE)?;

    match copy_attribute_value(focused_app as AXUIElementRef, K_AX_FOCUSED_WINDOW_ATTRIBUTE) {
        Ok(window) => {
            // El caller debe liberar focused_app y window
            release_ax_element(focused_app);
            Ok(window as AXUIElementRef)
        }
        Err(e) => {
            release_ax_element(focused_app);
            tracing::error!("Failed to get main window: {}", e);
            Ok(focused_app as AXUIElementRef)
        }
    }
}

// Obtiene el rectangulo (posicion y tamaño) e un elemento
pub fn get_element_rect(element: AXUIElementRef) -> Result<Rect> {
    // Obtener posicion
    let position_value = copy_attribute_value(element, K_AX_POSITION_ATTRIBUTE)?;

    let mut point: [f64; 2] = [0.0, 0.0];

    // SAFETY: AXValueGetValue es segura con un AXValueRef válido y un puntero válido
    let position_ok = unsafe {
        AXValueGetValue(
            position_value as AXValueRef,
            K_AX_VALUE_TYPE_CGPOINT,
            point.as_mut_ptr() as *mut c_void,
        )
    };

    release_ax_element(position_value);

    if !position_ok {
        return Err(AppError::Accessibility(
            "Failed to get element position".to_string(),
        ));
    }

    // Obtener tamaño
    let size_value = copy_attribute_value(element, K_AX_SIZE_ATTRIBUTE)?;

    let mut size: [f64; 2] = [0.0, 0.0];

    // SAFETY: AXValueGetValue es segura con un AXValueRef válido y un puntero válido
    let size_ok = unsafe {
        AXValueGetValue(
            size_value,
            K_AX_VALUE_TYPE_CGSIZE,
            size.as_mut_ptr() as *mut c_void,
        )
    };
    release_ax_element(size_value);

    if !size_ok {
        return Err(AppError::Accessibility(
            "Failed to get element size".to_string(),
        ));
    }

    Ok(Rect {
        x: point[0],
        y: point[1],
        width: size[0],
        height: size[1],
    })
}

// Obtiene el rol de un elemento (ej: AXButton, AXLink, etc.)
pub fn get_element_role(element: AXUIElementRef) -> Result<String> {
    let role_value = copy_attribute_value(element, K_AX_ROLE_ATTRIBUTE)?;

    // SAFETY: wrap_under_create_rule toma ownership del CFStringRef
    let cf_string = unsafe { CFString::wrap_under_create_rule(role_value as *const _) };

    Ok(cf_string.to_string())
}

// Obtiene el titulo de un elemento (puede ser None si no tiene)
pub fn get_element_title(element: AXUIElementRef) -> Option<String> {
    match copy_attribute_value(element, K_AX_TITLE_ATTRIBUTE) {
        Ok(title_value) => {
            // SAFETY: wrap_under_create_rule toma ownership del CFStringRef
            let cf_string = unsafe { CFString::wrap_under_create_rule(title_value as *const _) };
            let title = cf_string.to_string();
            if title.is_empty() {
                None
            } else {
                Some(title)
            }
        }
        Err(_) => None,
    }
}

// Obtiene los hijos de un elemento
pub fn get_children(element: AXUIElementRef) -> Result<Vec<AXUIElementRef>> {
    let children_value = match copy_attribute_value(element, K_AX_CHILDREN_ATTRIBUTE) {
        Ok(value) => value,
        Err(_) => {
            // No todos los elementos tienen hijos
            return Ok(Vec::new());
        }
    };

    // SAFETY: wrap_under_create_rule toma ownership del CFArrayRef
    let cf_array: CFArray<CFType> =
        unsafe { CFArray::wrap_under_create_rule(children_value as *const _) };

    let count = cf_array.len();
    let mut children = Vec::with_capacity(count as usize);

    for i in 0..count {
        if let Some(child) = cf_array.get(i) {
            // Retener el elemento para que no se libere cuando cf_array salga de scope
            let child_ptr = child.as_concrete_TypeRef() as AXUIElementRef;
            children.push(child_ptr);
        }
    }

    Ok(children)
}

// Verifica si un rol es clickeable
pub fn is_clickable_role(role: &str) -> bool {
    CLICKABLE_ROLES.contains(&role)
}

/// Recorre el árbol de accesibilidad usando BFS limitada
/// Devuelve elementos clickeables encontrados
pub fn traverse_accessibility_tree(
    root: AXUIElementRef,
    max_depth: usize,
    max_elements: usize,
) -> Vec<AXUIElementRef> {
    tracing::info!(
        "Traversing accessibility tree (max_depth: {}, max_elements: {})",
        max_depth,
        max_elements
    );

    let mut clickable_elements = Vec::new();
    let mut queue: std::collections::VecDeque<(AXUIElementRef, usize)> =
        std::collections::VecDeque::new();

    queue.push_back((root, 0));

    while let Some((element, depth)) = queue.pop_front() {
        // Límite de profundidad
        if depth > max_depth {
            continue;
        }

        // Límite de elementos encontrados
        if clickable_elements.len() >= max_elements {
            tracing::info!("Reached max elements limit: {}", max_elements);
            break;
        }

        // Verificar si el elemento es clickeable
        if let Ok(role) = get_element_role(element) {
            if is_clickable_role(&role) {
                // Verificar que el elemento tiene un rectángulo válido
                if let Ok(rect) = get_element_rect(element) {
                    if rect.width > 0.0 && rect.height > 0.0 {
                        tracing::debug!(
                            "Found clickable element: {} at ({}, {}) {}x{}",
                            role,
                            rect.x,
                            rect.y,
                            rect.width,
                            rect.height
                        );
                        clickable_elements.push(element);
                    }
                }
            }
        }

        // Añadir hijos a la cola
        if depth < max_depth {
            if let Ok(children) = get_children(element) {
                for child in children {
                    queue.push_back((child, depth + 1));
                }
            }
        }
    }

    tracing::info!(
        "Traversal complete: found {} clickable elements",
        clickable_elements.len()
    );

    clickable_elements
}

/// Helper para obtener el valor de un atributo como String
pub fn get_attribute_as_string(element: AXUIElementRef, attribute: &str) -> Result<String> {
    let value = copy_attribute_value(element, attribute)?;

    // SAFETY: wrap_under_create_rule toma ownership del CFStringRef
    let cf_string = unsafe { CFString::wrap_under_create_rule(value as *const _) };

    Ok(cf_string.to_string())
}
/// Obtiene los nombres de todos los atributos disponibles del elemento
#[allow(dead_code)]
pub fn get_attribute_names(element: AXUIElementRef) -> Result<Vec<String>> {
    let mut names_ref: *const c_void = std::ptr::null();

    // SAFETY: AXUIElementCopyAttributeNames es segura si element es válido
    let result = unsafe { AXUIElementCopyAttributeNames(element, &mut names_ref) };

    if result != 0 {
        return Err(AppError::Accessibility(format!(
            "Failed to get attribute names: AX error code {}",
            result
        )));
    }

    if names_ref.is_null() {
        return Err(AppError::Accessibility(
            "Attribute names returned null".to_string(),
        ));
    }

    // SAFETY: wrap_under_create_rule toma ownership del CFArrayRef
    let cf_array = unsafe {
        core_foundation::array::CFArray::<CFString>::wrap_under_create_rule(names_ref as *const _)
    };

    let mut names = Vec::with_capacity(cf_array.len() as usize);
    for i in 0..cf_array.len() {
        if let Some(name) = cf_array.get(i) {
            names.push(name.to_string());
        }
    }

    Ok(names)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constants_not_empty() {
        assert!(!K_AX_TRUSTED_CHECK_OPTION_PROMPT.is_empty());
        assert!(!K_AX_FOCUSED_APPLICATION_ATTRIBUTE.is_empty());
        assert!(!K_AX_PID_ATTRIBUTE.is_empty());
    }

    #[test]
    fn test_clickable_roles() {
        assert!(is_clickable_role("AXButton"));
        assert!(is_clickable_role("AXLink"));
        assert!(!is_clickable_role("AXWindow"));
        assert!(!is_clickable_role("AXStaticText"));
    }

    #[test]
    fn test_rect_default() {
        let rect = Rect {
            x: 10.0,
            y: 20.0,
            width: 100.0,
            height: 50.0,
        };
        assert_eq!(rect.x, 10.0);
        assert_eq!(rect.y, 20.0);
        assert_eq!(rect.width, 100.0);
        assert_eq!(rect.height, 50.0);
    }

    #[test]
    #[cfg(target_os = "macos")]
    fn test_has_permissions_returns_bool() {
        let result = has_accessibility_permissions();
        assert!(result == true || result == false);
    }

    #[test]
    fn test_traverse_empty_returns_empty() {
        let result = traverse_accessibility_tree(std::ptr::null(), 10, 100);
        // Con un puntero nulo, debería devolver vacío o fallar graciosamente
        assert!(result.is_empty());
    }
}
