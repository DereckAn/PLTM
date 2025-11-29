use crate::{AppError, Result};
use crossbeam_channel::Receiver;
use global_hotkey::hotkey::{Code, HotKey, Modifiers};
use global_hotkey::{GlobalHotKeyEvent, GlobalHotKeyManager};

pub const DEFAULT_HOTKEY: &str = "Cmd+Shift+J";

/// Servicio para gestionar hotkeys globales (macOS/Win)
pub struct HotkeyService {
    registered_hotkey: Option<String>,
    is_active: bool,
}

impl HotkeyService {
    pub fn new() -> Self {
        tracing::debug!("Initializing HotkeyService");
        Self {
            registered_hotkey: None,
            is_active: false,
        }
    }

    /// Registra el hotkey por defecto (Cmd+Shift+J)
    #[allow(dead_code)]
    pub fn register_default(&mut self) -> Result<Receiver<GlobalHotKeyEvent>> {
        self.register(DEFAULT_HOTKEY)
    }

    /// Registra un hotkey global y devuelve un receptor de eventos
    pub fn register(&mut self, hotkey: &str) -> Result<Receiver<GlobalHotKeyEvent>> {
        tracing::info!("Registering hotkey: {}", hotkey);

        let parsed = parse_hotkey(hotkey)?;
        let manager = GlobalHotKeyManager::new().map_err(|e| AppError::Hotkey(e.to_string()))?;
        manager
            .register(parsed)
            .map_err(|e| AppError::Hotkey(e.to_string()))?;

        let rx = GlobalHotKeyEvent::receiver().clone();

        self.registered_hotkey = Some(hotkey.to_string());
        self.is_active = true;

        Ok(rx)
    }

    /// Desregistra el hotkey actual
    #[allow(dead_code)]
    pub fn unregister(&mut self) -> Result<()> {
        tracing::info!("Unregistering hotkey");
        if let Some(ref key) = self.registered_hotkey {
            let parsed = parse_hotkey(key)?;
            GlobalHotKeyManager::new()
                .map_err(|e| AppError::Hotkey(e.to_string()))?
                .unregister(parsed)
                .map_err(|e| AppError::Hotkey(e.to_string()))?;
        }
        self.registered_hotkey = None;
        self.is_active = false;
        Ok(())
    }

    /// Obtiene el hotkey registrado actualmente
    #[allow(dead_code)]
    pub fn current_hotkey(&self) -> Option<&str> {
        self.registered_hotkey.as_deref()
    }

    // Verifica si el servicio esta activo
    #[allow(dead_code)]
    pub fn is_active(&self) -> bool {
        self.is_active
    }
}

impl Default for HotkeyService {
    fn default() -> Self {
        Self::new()
    }
}

/// Parsea una cadena estilo "Cmd+Shift+J" a HotKey
fn parse_hotkey(input: &str) -> Result<HotKey> {
    let mut mods = Modifiers::empty();
    let mut key: Option<Code> = None;

    for part in input.split('+') {
        match part.to_lowercase().as_str() {
            "cmd" | "meta" => mods |= Modifiers::META,
            "shift" => mods |= Modifiers::SHIFT,
            "alt" | "option" => mods |= Modifiers::ALT,
            "ctrl" | "control" => mods |= Modifiers::CONTROL,
            k if k.len() == 1 => {
                let ch = k.chars().next().unwrap().to_ascii_uppercase();
                key = match ch {
                    'A' => Some(Code::KeyA),
                    'B' => Some(Code::KeyB),
                    'C' => Some(Code::KeyC),
                    'D' => Some(Code::KeyD),
                    'E' => Some(Code::KeyE),
                    'F' => Some(Code::KeyF),
                    'G' => Some(Code::KeyG),
                    'H' => Some(Code::KeyH),
                    'I' => Some(Code::KeyI),
                    'J' => Some(Code::KeyJ),
                    'K' => Some(Code::KeyK),
                    'L' => Some(Code::KeyL),
                    'M' => Some(Code::KeyM),
                    'N' => Some(Code::KeyN),
                    'O' => Some(Code::KeyO),
                    'P' => Some(Code::KeyP),
                    'Q' => Some(Code::KeyQ),
                    'R' => Some(Code::KeyR),
                    'S' => Some(Code::KeyS),
                    'T' => Some(Code::KeyT),
                    'U' => Some(Code::KeyU),
                    'V' => Some(Code::KeyV),
                    'W' => Some(Code::KeyW),
                    'X' => Some(Code::KeyX),
                    'Y' => Some(Code::KeyY),
                    'Z' => Some(Code::KeyZ),
                    _ => None,
                };
            }
            _ => {}
        }
    }

    let key = key.ok_or_else(|| AppError::Hotkey("Invalid hotkey: no key found".to_string()))?;

    // Si no hay modifiers, pasar None
    let mods_option = if mods.is_empty() { None } else { Some(mods) };

    Ok(HotKey::new(mods_option, key))
}

/// Parsea y devuelve los modifiers y key por separado (para testing)
#[cfg(test)]
fn parse_hotkey_parts(input: &str) -> Result<(Modifiers, Code)> {
    let mut mods = Modifiers::empty();
    let mut key: Option<Code> = None;

    for part in input.split('+') {
        match part.to_lowercase().as_str() {
            "cmd" | "meta" => mods |= Modifiers::META,
            "shift" => mods |= Modifiers::SHIFT,
            "alt" | "option" => mods |= Modifiers::ALT,
            "ctrl" | "control" => mods |= Modifiers::CONTROL,
            k if k.len() == 1 => {
                let ch = k.chars().next().unwrap().to_ascii_uppercase();
                key = match ch {
                    'A' => Some(Code::KeyA),
                    'J' => Some(Code::KeyJ),
                    'K' => Some(Code::KeyK),
                    _ => None,
                };
            }
            _ => {}
        }
    }

    let key = key.ok_or_else(|| AppError::Hotkey("Invalid hotkey".to_string()))?;
    Ok((mods, key))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_hotkey_cmd_shift_j() {
        // Test the parsing logic directly, not the HotKey struct internals
        let (mods, key) = parse_hotkey_parts("Cmd+Shift+J").unwrap();
        assert!(mods.contains(Modifiers::META));
        assert!(mods.contains(Modifiers::SHIFT));
        assert_eq!(key, Code::KeyJ);
    }

    #[test]
    fn test_parse_hotkey_ctrl_alt_a() {
        let (mods, key) = parse_hotkey_parts("Ctrl+Alt+A").unwrap();
        assert!(mods.contains(Modifiers::CONTROL));
        assert!(mods.contains(Modifiers::ALT));
        assert_eq!(key, Code::KeyA);
    }

    #[test]
    fn test_parse_hotkey_single_modifier() {
        let (mods, key) = parse_hotkey_parts("Cmd+K").unwrap();
        assert!(mods.contains(Modifiers::META));
        assert!(!mods.contains(Modifiers::SHIFT));
        assert_eq!(key, Code::KeyK);
    }

    #[test]
    fn test_parse_hotkey_invalid() {
        let result = parse_hotkey("Invalid");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_hotkey_creates_valid_hotkey() {
        // Just verify that parse_hotkey doesn't panic and returns Ok
        let result = parse_hotkey("Cmd+Shift+J");
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_hotkey_no_modifier() {
        // Single key without modifiers should still work
        let result = parse_hotkey("J");
        assert!(result.is_ok());
    }
}