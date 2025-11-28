use serde::{Deserialize, Serialize};

// Configuracion principal de la aplicacion
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppConfig {
    // Hotkey para activar navegacion (ej: "Cmd+J")
    pub hotkey: String,

    // Profundidad maxima de escaneo del arbol de accesibilidad
    pub scan_depth: usize,

    // Numero maximo de elementos a escanear
    pub max_elements: usize,

    // Caracteres usados para generar hints (home row keys)
    pub hint_chars: String,

    // Timeout en ms para la secuencia de hints
    pub hint_timeout: u64,
}

impl Default for AppConfig {
    fn default() -> Self {
        AppConfig {
            hotkey: "Cmd+J".to_string(),
            scan_depth: 10,
            max_elements: 500,
            hint_chars: "asdfghjkl".to_string(),
            hint_timeout: 1000,
        }
    }
}

impl AppConfig {
    pub fn new() -> Self {
        Self::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = AppConfig::default();
        assert_eq!(config.hotkey, "Cmd+J");
        assert_eq!(config.scan_depth, 10);
        assert_eq!(config.max_elements, 500);
        assert_eq!(config.hint_chars, "asdfghjkl");
        assert_eq!(config.hint_timeout, 1000);
    }
}
