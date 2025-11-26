use tauri::State;

use crate::state::AppState;

#[tauri::command]
pub async fn register_hotkey(key_combo: String, state: State<'_, AppState>) -> Result<(), String> {
    let mut hotkeys = state.hotkey_service.lock().await;

    hotkeys
        .register_activation_hotkey(&key_combo)
        .map_err(|e| e.to_string())
}
