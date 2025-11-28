use crate::Result;

pub const DEFAULT_HOTKEY: &str = "Cmd+J";
/// Servicio para gestionar hotkeys globales
/// TODO: Implementar captura real de eventos de teclado en Fase 2
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

    // Registra el hotkey por defecto (cmd+j)
    pub fn register_default(&mut self) -> Result<()> {
        self.register(DEFAULT_HOTKEY)
    }


    /// Registra un hotkey global
    /// TODO: Implementar registro real con APIs nativas por OS
    pub fn register(&mut self, hotkey: &str) -> Result<()> {
        tracing::info!("Registering hotkey: {}", hotkey);
        // TODO: Parsear hotkey string (ej: "Cmd+J" -> modifiers + key)
        // TODO: Llamar a API nativa para registrar el hotkey global
        self.registered_hotkey = Some(hotkey.to_string());
        self.is_active = true;
        Ok(())
    }

    /// Desregistra el hotkey actual
    #[allow(dead_code)]
    pub fn unregister(&mut self) -> Result<()> {
        tracing::info!("Unregistering hotkey");
        // TODO: Llamar a API nativa para desregistrar
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_hotkey() {
        let mut service = HotkeyService::new();
        assert!(service.register("Cmd+J").is_ok());
        assert_eq!(service.current_hotkey(), Some("Cmd+J"));
        assert!(service.is_active());
    }

    #[test]
    fn test_register_default_hotkey() {
        let mut service = HotkeyService::new();
        assert!(service.register("Cmd+K").is_ok());
        assert_eq!(service.current_hotkey(), Some("Cmd+K"));
        assert!(service.is_active());
    }

    #[test]
    fn test_unregister_hotkey() {
        let mut service = HotkeyService::new();
        service.register("Cmd+J").unwrap();
        assert!(service.unregister().is_ok());
        assert_eq!(service.current_hotkey(), None);
        assert!(!service.is_active());
    }
}
