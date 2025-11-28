use tauri::State;

use crate::state::AppState;
use crate::Result;

#[tauri::command]
pub async fn register_hotkey(key_combo: String, state: State<'_, AppState>) -> Result<()> {
    let mut hotkeys = state.hotkey_service.lock().await;

    hotkeys.register(&key_combo)
}
