#[tauri::command]
pub async fn register_hotkey(
    key_combo: String,
    state: State<'_, AppState>,
) -> Result<(), String> {
    state.hotkey_service
    .lock()
    .await
    .register_activation_hotkey(&key_combo)
    .map_err(|e| e.to_string())
}