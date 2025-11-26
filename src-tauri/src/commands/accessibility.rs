use tauri::State;

use crate::models::UIElement;
use crate::state::AppState;

#[tauri::command]
pub async fn scan_elements(state: State<'_, AppState>) -> Result<Vec<UIElement>, String> {
    state
        .accessibility_service
        .scan_clickable_elements()
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn check_permissions(state: State<'_, AppState>) -> Result<bool, String> {
    Ok(state.accessibility_service.check_permissions())
}

#[tauri::command]
pub async fn request_permissions(state: State<'_, AppState>) -> Result<(), String> {
    state
        .accessibility_service
        .request_permissions()
        .map_err(|e| e.to_string())
}
