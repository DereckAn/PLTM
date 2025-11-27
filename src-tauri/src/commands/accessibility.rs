use tauri::State;

use crate::models::UIElement;
use crate::state::AppState;
use crate::Result;

#[tauri::command]
pub async fn scan_elements(state: State<'_, AppState>) -> Result<Vec<UIElement>> {
    tracing::info!("Scanning clickable elements");
    state.accessibility_service.scan_clickable_elements().await
}

#[tauri::command]
pub async fn check_permissions(state: State<'_, AppState>) -> Result<bool> {
    tracing::info!("Checking accessibility permissions");
    Ok(state.accessibility_service.check_permissions())
}

#[tauri::command]
pub async fn request_permissions(state: State<'_, AppState>) -> Result<()> {
    tracing::info!("Requesting accessibility permissions");
    state.accessibility_service.request_permissions()
}
