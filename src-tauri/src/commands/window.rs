use tauri::{AppHandle, State};

use crate::models::Hint;
use crate::state::AppState;

#[tauri::command]
pub async fn show_hints(
    hints: Vec<Hint>,
    _app: AppHandle,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let mut wm = state.window_manager.lock().await;

    wm.show_overlay(&hints).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn perform_click(x: f64, y: f64, state: State<'_, AppState>) -> Result<(), String> {
    state
        .click_service
        .perform_click(x, y)
        .map_err(|e| e.to_string())
}
