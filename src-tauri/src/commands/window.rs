use tauri::{AppHandle, State};
use std::sync::mpsc;

use crate::models::Hint;
use crate::state::AppState;
use crate::Result;
use crate::AppError;

#[tauri::command]
pub async fn show_hints(
    hints: Vec<Hint>,
    app: AppHandle,
    state: State<'_, AppState>,
) -> Result<()> {
    tracing::info!("Command: show_hints called with {} hints", hints.len());

    // Guard: Verificar permisos antes de mostrar hints
    if let Err(e) = state.accessibility_service.ensure_permissions() {
        tracing::error!("Cannot show hints without accessibility permissions: {}", e);
        return Err(e);
    }

    let wm = state.window_manager.clone();
    let (tx, rx) = mpsc::channel();

    app
        .run_on_main_thread(move || {
            let res = tauri::async_runtime::block_on(async {
                let mut guard = wm.lock().await;
                guard.show_overlay(&hints).await
            });
            let _ = tx.send(res);
        })
        .map_err(|e| {
            tracing::error!("Failed to render overlay on main thread: {}", e);
            AppError::Overlay("Failed to render overlay".to_string())
        })?;

    let _render_res = rx
        .recv()
        .map_err(|_| AppError::Overlay("Failed to render overlay".to_string()))??;

    Ok(())
}

#[tauri::command]
pub async fn perform_click(x: f64, y: f64, state: State<'_, AppState>) -> Result<()> {
    tracing::info!("Command: perform_click at coordinates ({}, {})", x, y);
    state.click_service.perform_click(x, y)
}
