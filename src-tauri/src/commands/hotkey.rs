use std::thread;
use tauri::{AppHandle, Emitter, State};

use crate::state::AppState;
use crate::Result;

#[tauri::command]
pub async fn register_hotkey(
    key_combo: String,
    app: AppHandle,
    state: State<'_, AppState>,
) -> Result<()> {
    let mut hotkeys = state.hotkey_service.lock().await;

    let rx = hotkeys.register(&key_combo)?;

    // Escuchar el hotkey en un hilo dedicado y emitir evento a frontend
    thread::spawn(move || {
        for _event in rx.iter() {
            let _ = app.emit("hotkey-activate", ());
        }
    });

    Ok(())
}
