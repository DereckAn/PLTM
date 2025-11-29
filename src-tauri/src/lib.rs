mod commands;
mod error;
mod models;
pub mod platform;
mod services;
mod state;

use crate::commands::accessibility::get_focused_app_pid;
use crate::commands::*;
use crate::error::AppError;
use crate::state::app_state::AppState;
use tracing_subscriber::EnvFilter;

pub type Result<T> = std::result::Result<T, AppError>;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .init();

    tracing::info!("Starting Tauri application...");

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(AppState::new())
        .invoke_handler(tauri::generate_handler![
            scan_elements,
            check_permissions,
            request_permissions,
            open_accessibility_settings,
            register_hotkey,
            get_focused_app_pid,
            show_hints,
            perform_click,
            activate_navigation,
            deactivate_navigation,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
