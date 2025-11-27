use crate::error::AppError;
use crate::models::UIElement;
use crate::platform::macos::accessibility;
use crate::Result;

pub struct AccessibilityService;

impl AccessibilityService {
    pub fn new() -> Self {
        tracing::info!("Initializing AccessibilityService");
        Self
    }

    /// Verifica si la aplicación tiene permisos de accesibilidad
    pub fn check_permissions(&self) -> bool {
        tracing::info!("AccessibilityService checking permissions");

        let has_perms = accessibility::has_accessibility_permissions();

        if has_perms {
            tracing::info!("Accessibility permissions are granted");
        } else {
            tracing::warn!("Accessibility permissions are NOT granted");
        }

        has_perms
    }

    /// Solicita permisos de accesibilidad al usuario
    pub fn request_permissions(&self) -> Result<()> {
        tracing::info!("AccessibilityService requesting permissions");

        match accessibility::request_permissions() {
            Ok(_) => {
                tracing::info!("Permission request completed successfully");
                Ok(())
            }
            Err(e) => {
                tracing::error!("Failed to request accessibility permissions: {}", e);
                Err(e)
            }
        }
    }

    /// Obtiene el PID de la aplicación actualmente enfocada
    pub fn get_focused_application_pid(&self) -> Result<Option<i32>> {
        tracing::info!("AccessibilityService getting focused application PID");

        match accessibility::get_focused_application() {
            Ok(Some(pid)) => {
                tracing::info!("Found focused application with PID: {}", pid);
                Ok(Some(pid))
            }
            Ok(None) => {
                tracing::warn!("No focused application found");
                Ok(None)
            }
            Err(e) => {
                tracing::error!("Failed to get focused application: {}", e);
                Err(e)
            }
        }
    }

    /// Verifica permisos y retorna error si no están disponibles
    /// Guard function para usar al inicio de operaciones que requieren accesibilidad
    pub fn ensure_permissions(&self) -> Result<()> {
        tracing::info!("AccessibilityService ensuring permissions");

        if self.check_permissions() {
            tracing::info!("Permissions verified successfully");
            Ok(())
        } else {
            tracing::error!("Accessibility permissions check failed - access denied");
            Err(AppError::Accessibility(
        "Accessibility permissions are not granted. Please enable in System Preferences > Privacy & Security > Accessibility".to_string(),
      ))
        }
    }

    /// Escanea elementos clicables en la aplicación activa
    pub async fn scan_clickable_elements(&self) -> Result<Vec<UIElement>> {
        tracing::info!("AccessibilityService scanning clickable elements");

        // Guard: verificar permisos antes de continuar
        self.ensure_permissions()?;

        // TODO: Implementar en próximos pasos:
        // 1. Obtener ventana activa con get_active_window()
        // 2. Recorrer árbol con traverse_accessibility_tree()
        // 3. Filtrar elementos clicables
        // 4. Convertir AXUIElementRef a UIElement

        tracing::warn!("scan_clickable_elements is not yet fully implemented (stub)");
        Ok(Vec::new())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_service_creation() {
        let service = AccessibilityService::new();
        assert!(std::mem::size_of_val(&service) == 0, "Should be zero-sized");
    }

    #[test]
    #[cfg(target_os = "macos")]
    fn test_check_permissions_returns_bool() {
        let service = AccessibilityService::new();
        let result = service.check_permissions();
        assert!(result == true || result == false);
    }

    #[test]
    #[cfg(target_os = "macos")]
    fn test_ensure_permissions_returns_result() {
        let service = AccessibilityService::new();
        let result = service.ensure_permissions();
        // Pasa si tiene permisos (Ok) o si falla por falta de permisos (Err)
        assert!(result.is_ok() || result.is_err());
    }

    #[tokio::test]
    async fn test_scan_stub_returns_empty_or_permission_error() {
        let service = AccessibilityService::new();
        let result = service.scan_clickable_elements().await;
        // Puede ser Ok (vacío) o Err (sin permisos)
        match result {
            Ok(elements) => assert!(elements.is_empty()),
            Err(e) => assert!(e.to_string().contains("Accessibility")),
        }
    }
}
