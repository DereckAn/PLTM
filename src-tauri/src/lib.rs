mod commands;
mod error;
mod models;
mod platform;
mod services;
mod state;

use crate::commands::*;
use crate::error::AppError;
use crate::state::app_state::AppState;

use tracing_subscriber::EnvFilter;

pub type Result<T> = std::result::Result<T, AppError>;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Iniciar tracing con nivel info por defecto
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .init();
    tracing::info!("Starting Tauri application...");

    let app_state = match AppState::new() {
        Ok(state) => state,
        Err(err) => {
            tracing::error!("Failed to initialize AppState: {}", err);
            panic!("AppState init failed: {}", err);
        }
    };

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(app_state)
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
