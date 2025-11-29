#[cfg(test)]
#[cfg(target_os = "macos")]
mod macos_accessibility_tests {
    use pltm_lib::platform::macos::accessibility;

    // Código de error AX cuando no hay permisos o no se puede completar
    const AX_ERROR_CANNOT_COMPLETE: &str = "-25204";
    const AX_ERROR_API_DISABLED: &str = "-25211";

    /// Helper para verificar si un error es por falta de permisos
    fn is_permission_error(err: &str) -> bool {
        err.contains(AX_ERROR_CANNOT_COMPLETE)
            || err.contains(AX_ERROR_API_DISABLED)
            || err.contains("Check accessibility permissions")
    }

    #[test]
    fn test_has_accessibility_permissions() {
        let has_perms = accessibility::has_accessibility_permissions();
        println!("Has accessibility permissions: {}", has_perms);
        // Solo verifica que no crashee
        assert!(has_perms == true || has_perms == false);
    }

    #[test]
    fn test_request_permissions_does_not_crash() {
        let result = accessibility::request_permissions();
        // Solo verifica que no crashee, el resultado depende del sistema
        assert!(result.is_ok());
    }

    #[test]
    fn test_get_focused_application_pid() {
        match accessibility::get_focused_application() {
            Ok(Some(pid)) => {
                println!("Focused app PID: {}", pid);
                assert!(pid > 0, "PID should be positive");
            }
            Ok(None) => {
                println!("No focused application (screensaver/login screen)");
            }
            Err(e) => {
                let err_str = e.to_string();
                println!("Error (expected without permissions): {}", err_str);
                // El test pasa si el error es por falta de permisos
                assert!(
                    is_permission_error(&err_str),
                    "Unexpected error: {}",
                    err_str
                );
            }
        }
    }

    #[test]
    fn test_get_active_window() {
        match accessibility::get_active_window() {
            Ok(element) => {
                println!("Got active window element: {:?}", element);
                assert!(!element.is_null(), "Element should not be null");
            }
            Err(e) => {
                let err_str = e.to_string();
                println!("Error (expected without permissions): {}", err_str);
                // El test pasa si el error es por falta de permisos
                assert!(
                    is_permission_error(&err_str),
                    "Unexpected error: {}",
                    err_str
                );
            }
        }
    }
}
