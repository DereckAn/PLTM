mod commands;
mod models;
mod platform;
mod services;
mod state;

use crate::commands::*;
use crate::state::app_state::AppState;

pub type Result<T> = std::result::Result<T, String>;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(AppState::new())
        .invoke_handler(tauri::generate_handler![
            scan_elements,
            check_permissions,
            request_permissions,
            register_hotkey,
            show_hints,
            perform_click
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
