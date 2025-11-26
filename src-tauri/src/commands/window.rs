#[tauri::command]
pub async fn show_hints(
    hints: Vec<Hint>,
    app: Apphandle,
    state: State<'_, AppState>,
) -> Result<(), String> {
    state
        .window_manager
        .lock()
        .await
        .show_overlay(hints)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn perform_click(x: f64, y: f64, state: State<'_, AppState>) -> Result<(), String> {
    state
        .click_service
        .perform_click(x, y)
        .map_err(|e| e.to_string())
}
