mod commands;
mod error;
mod models;
pub mod platform;
mod services;
mod state;

use std::thread;

use crate::commands::accessibility::get_focused_app_pid;
use crate::commands::*;
use crate::error::AppError;
use crate::services::hotkey_service::DEFAULT_HOTKEY;
use crate::state::app_state::AppState;
use global_hotkey::HotKeyState;
use tauri::{Emitter, Manager};
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
        .setup(|app| {
            // Registrar hotkey global por defecto al inicio
            let app_handle = app.handle();
            let state = app_handle.state::<AppState>();

            tauri::async_runtime::block_on(async {
                let mut hotkeys = state.hotkey_service.lock().await;
                match hotkeys.register(DEFAULT_HOTKEY) {
                    Ok(rx) => {
                        tracing::info!("Default hotkey registered at startup: {}", DEFAULT_HOTKEY);
                        let app_handle = app_handle.clone();
                        thread::spawn(move || {
                            for event in rx.iter() {
                                if event.state == HotKeyState::Pressed {
                                    if let Err(e) = app_handle.emit("hotkey-activate", ()) {
                                        tracing::error!("Failed to emit hotkey-activate: {}", e);
                                    }
                                }
                            }
                        });
                    }
                    Err(e) => {
                        tracing::error!(
                            "Failed to register default hotkey ({}): {}",
                            DEFAULT_HOTKEY,
                            e
                        );
                    }
                }
            });

            Ok(())
        })
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
