use crate::{error::AppError, Result};
use global_hotkey::hotkey::{Code, HotKey, Modifiers};
use global_hotkey::GlobalHotKeyManager;
use std::collections::HashMap;
use tauri::AppHandle;

pub struct HotkeyService {
    manager: GlobalHotKeyManager,
    registered_hotkeys: HashMap<String, HotKey>,
}

impl HotkeyService {
    pub fn new() -> Result<Self> {
        tracing::info!("Initializing HotkeyService...");

        let manager = GlobalHotKeyManager::new().map_err(|e| {
            tracing::error!("Failed to initialize hotkey manager: {}", e);
            AppError::Hotkey(e.to_string())
        })?;

        tracing::info!("HotkeyService initialized.");
        Ok(Self {
            manager,
            registered_hotkeys: HashMap::new(),
        })
    }

    // Registra el hotkey principal para activar la navegacion
    pub fn register_activation_hotkey(&mut self, key_combo: &str) -> Result<()> {
        tracing::info!("Registering activation hotkey: {}", key_combo);

        // 1. Parsear key_combo
        // 2. Registrar hotkey con GlobalHotKeyManager
        // 3. Almacenar en registered_hotkeys

        // Ej: "Cmd+J"

        let hotkey = HotKey::new(Some(Modifiers::SUPER), Code::KeyJ);
        match self.manager.register(hotkey) {
            Ok(_) => {
                self.registered_hotkeys
                    .insert(key_combo.to_string(), hotkey);
                tracing::info!("Hotkey '{}' registered successfully", key_combo);
                Ok(())
            }
            Err(e) => {
                tracing::error!("Failed to register hotkey '{}': {}", key_combo, e);
                Err(e.into())
            }
        }
    }

    // Maneja el evento cuando se presiona el hotkey
    #[allow(dead_code)]
    pub async fn handle_hotkey_pressed(&self, _app: AppHandle) {
        // TODO:  Activar modo navegacion, escanear, generar hints, mostrar overlay.
    }
}
