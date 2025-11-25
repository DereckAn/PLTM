use crate::models::elemtent::UIElement;
use crate::platform::macos::accessibility;

pub struct AccessibilityService {
    enabled: bool,
    cache: ElementCache,
}

impl AccessibilityService {
    pub fn new() -> Self {}

    /// Escanea todos los elementos clickeables en la pantalla 
    pub async fn scan_clickable_elements(&self) -> Result<Vec<UIElement>> {
        // 1. Verificar permisos 
        // 2. Obtener ventana activa
        // 3. Recorrer arbol de accessibilidad
        // 4. Filtrar elementos clickeables
        // 5. Calcular posiciones en pantalla
    }

    /// Verfica si tenemos permisos de accesibilidad
    pub fn check_permissions(&self) -> bool {
        accessibility::has_accessibility_permissions()
    }

    /// Solicita permisos al usuario
    pub fn request_permissions(&self) -> Result<()> {
        accessibility::request_permissions()
    }
}