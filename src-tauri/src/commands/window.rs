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
    tracing::info!("Command: show_hints called with {} hints", hints.len());

    // Guard: Verificar permisos antes de mostrar hints
    if let Err(e) = state.accessibility_service.ensure_permissions() {
        tracing::error!("Cannot show hints without accessibility permissions: {}", e);
        return Err(e);
    }

    let mut wm = state.window_manager.lock().await;

    wm.show_overlay(&hints).await
}

#[tauri::command]
pub async fn perform_click(x: f64, y: f64, state: State<'_, AppState>) -> Result<()> {
    tracing::info!("Command: perform_click at coordinates ({}, {})", x, y);
    state.click_service.perform_click(x, y)
}
