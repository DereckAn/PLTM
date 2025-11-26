use crate::Result;
use global_hotkey::hotkey::{Code, HotKey, Modifiers};
use global_hotkey::GlobalHotKeyManager;
use std::collections::HashMap;
use tauri::AppHandle;

pub struct HotkeyService {
    manager: GlobalHotKeyManager,
    registered_hotkeys: HashMap<String, HotKey>,
}

impl HotkeyService {
    pub fn new() -> Self {
        Self {
            manager: GlobalHotKeyManager::new().expect("hotkey manager"),
            registered_hotkeys: HashMap::new(),
        }
    }

    // Registra el hotkey principal para activar la navegacion
    pub fn register_activation_hotkey(&mut self, key_combo: &str) -> Result<()> {
        // 1. Parsear key_combo
        // 2. Registrar hotkey con GlobalHotKeyManager
        // 3. Almacenar en registered_hotkeys

        // Ej: "Cmd+J"

        let hotkey = HotKey::new(Some(Modifiers::SUPER), Code::KeyJ);
        self.manager
            .register(hotkey)
            .map_err(|e| e.to_string())?;
        self.registered_hotkeys
            .insert(key_combo.to_string(), hotkey);
        Ok(())
    }

    // Maneja el evento cuando se presiona el hotkey
    pub async fn handle_hotkey_pressed(&self, _app: AppHandle) {
        // TODO:  Activar modo navegacion, escanear, generar hints, mostrar overlay.
    }
}
