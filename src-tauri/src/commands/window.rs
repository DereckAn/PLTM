use tauri::{AppHandle, State};

use crate::models::Hint;
use crate::state::AppState;
use crate::Result;

#[tauri::command]
pub async fn show_hints(
    hints: Vec<Hint>,
    _app: AppHandle,
    state: State<'_, AppState>,
) -> Result<()> {
    let mut wm = state.window_manager.lock().await;

    wm.show_overlay(&hints).await
}

#[tauri::command]
pub async fn perform_click(x: f64, y: f64, state: State<'_, AppState>) -> Result<()> {
    state.click_service.perform_click(x, y)
}
