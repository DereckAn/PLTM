use crate::Result;

/// Servicio para gestionar hotkeys globales
/// TODO: Implementar captura real de eventos de teclado en Fase 2
pub struct HotkeyService {
    registered_hotkey: Option<String>,
}

impl HotkeyService {
    pub fn new() -> Self {
        tracing::debug!("Initializing HotkeyService");
        Self {
            registered_hotkey: None,
        }
    }

    /// Registra un hotkey global
    /// TODO: Implementar registro real con APIs nativas por OS
    pub fn register(&mut self, hotkey: &str) -> Result<()> {
        tracing::info!("Registering hotkey: {}", hotkey);
        // TODO: Parsear hotkey string (ej: "Cmd+J" -> modifiers + key)
        // TODO: Llamar a API nativa para registrar el hotkey global
        self.registered_hotkey = Some(hotkey.to_string());
        Ok(())
    }

    /// Desregistra el hotkey actual
    #[allow(dead_code)]
    pub fn unregister(&mut self) -> Result<()> {
        tracing::info!("Unregistering hotkey");
        // TODO: Llamar a API nativa para desregistrar
        self.registered_hotkey = None;
        Ok(())
    }

    /// Obtiene el hotkey registrado actualmente
    #[allow(dead_code)]
    pub fn current_hotkey(&self) -> Option<&str> {
        self.registered_hotkey.as_deref()
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
    }

    #[test]
    fn test_unregister_hotkey() {
        let mut service = HotkeyService::new();
        service.register("Cmd+J").unwrap();
        assert!(service.unregister().is_ok());
        assert_eq!(service.current_hotkey(), None);
    }
}
