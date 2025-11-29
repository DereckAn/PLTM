use tauri::State;

use crate::models::UIElement;
use crate::state::AppState;
use crate::error::AppError;
use crate::Result;

#[tauri::command]
pub async fn scan_elements(state: State<'_, AppState>) -> Result<Vec<UIElement>> {
    tracing::info!("Scanning clickable elements");
    match state.accessibility_service.scan_clickable_elements().await {
        Ok(elements) => {
            tracing::info!("Scan elements returnin {} elements", elements.len());
            Ok(elements)
        } 
        Err(e) => {
            tracing::error!("Failed to scan clickable elements: {}", e);
            Err(e)
        }
    }
}

#[tauri::command]
pub async fn check_permissions(state: State<'_, AppState>) -> Result<bool> {
    tracing::info!("Checking accessibility permissions");
    Ok(state.accessibility_service.check_permissions())
}

#[tauri::command]
pub async fn request_permissions(state: State<'_, AppState>) -> Result<()> {
    tracing::info!("Requesting accessibility permissions");
    // state.accessibility_service.request_permissions()
    match state.accessibility_service.request_permissions() {
        Ok(_) => Ok(()),
        Err(e) => {
            tracing::error!("Failed to request accessibility permissions: {}", e);
            Err(e)
        }
    }
}

#[tauri::command]
pub async fn get_focused_app_pid(state: State<'_, AppState>) -> Result<Option<i32>> {
    tracing::info!("Getting focused application PID");
    // state.accessibility_service.get_focused_application_pid()
    match state.accessibility_service.get_focused_application_pid() {
        Ok(pid_opt) => Ok(pid_opt),
        Err(e) => {
            tracing::error!("Failed to get focused application PID: {}", e);
            Err(e)
        }
    }
}

/// Abre directamente la sección de Accesibilidad en Configuración (macOS)
#[tauri::command]
#[cfg(target_os = "macos")]
pub async fn open_accessibility_settings() -> Result<()> {
    tracing::info!("Opening Accessibility preferences (macOS)");
    std::process::Command::new("open")
        .arg("x-apple.systempreferences:com.apple.preference.security?Privacy_Accessibility")
        .spawn()
        .map_err(|e| {
            tracing::error!("Failed to open Accessibility settings: {}", e);
            AppError::Accessibility("Unable to open System Settings".to_string())
        })?;
    Ok(())
}

/// Stub para plataformas no soportadas
#[tauri::command]
#[cfg(not(target_os = "macos"))]
pub async fn open_accessibility_settings() -> Result<()> {
    Err(AppError::Accessibility(
        "Accessibility settings shortcut not supported on this OS".to_string(),
    ))
}
