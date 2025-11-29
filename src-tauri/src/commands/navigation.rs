use tauri::{AppHandle, State};
use std::sync::mpsc;

use crate::models::Hint;
use crate::services::{filter_elements, HintGenerator};
use crate::state::AppState;
use crate::Result;
use crate::AppError;

/// Comando que ejecuta el flujo completo: scan → filter → hints → overlay
#[tauri::command]
pub async fn activate_navigation(app: AppHandle, state: State<'_, AppState>) -> Result<Vec<Hint>> {
    tracing::info!("Command: activate_navigation");

    // 1. Verificar permisos
    state.accessibility_service.ensure_permissions()?;

    // 2. Escanear elementos clickables
    let elements = state
        .accessibility_service
        .scan_clickable_elements()
        .await?;
    tracing::info!("Scanned {} elements", elements.len());

    // 3. Filtrar elementos (tamaño mínimo, deduplicación)
    let filtered = filter_elements(elements);
    tracing::info!("Filtered to {} elements", filtered.len());

    // 4. Generar hints
    let hint_generator = HintGenerator::new();
    let hints = hint_generator.generate(&filtered);
    tracing::info!("Generated {} hints", hints.len());

    // 5. Mostrar overlay en hilo principal
    let wm = state.window_manager.clone();
    let hints_clone = hints.clone();
    let (tx, rx) = mpsc::channel();

    app
        .run_on_main_thread(move || {
            let res = tauri::async_runtime::block_on(async {
                let mut guard = wm.lock().await;
                guard.show_overlay(&hints_clone).await
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

    tracing::info!("Navigation activated successfully");
    Ok(hints)
}

/// Comando para desactivar la navegación y ocultar el overlay
#[tauri::command]
pub async fn deactivate_navigation(state: State<'_, AppState>) -> Result<()> {
    tracing::info!("Command: deactivate_navigation");

    let mut wm = state.window_manager.lock().await;
    wm.hide_overlay().await?;

    tracing::info!("Navigation deactivated");
    Ok(())
}
