use serde::Serialize;
use thiserror::Error;

#[derive(Error, Debug, Serialize)]
pub enum AppError {
    #[error("Accessibility error: {0}")]
    Accessibility(String),

    #[error("Hotkey error: {0}")]
    Hotkey(String),

    #[error("Overlay error: {0}")]
    Overlay(String),

    #[error("Click error: {0}")]
    Click(String),

    #[error("Config error: {0}")]
    Config(String),

    #[error("{0}")]
    Other(String),
}

// Conversiones automáticas desde errores comunes
impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        AppError::Config(format!("IO error: {}", err))
    }
}

impl From<serde_json::Error> for AppError {
    fn from(err: serde_json::Error) -> Self {
        AppError::Config(format!("JSON error: {}", err))
    }
}

impl From<global_hotkey::Error> for AppError {
    fn from(err: global_hotkey::Error) -> Self {
        AppError::Hotkey(err.to_string())
    }
}

// Para Tauri commands que devuelven Result<T, String>
impl From<AppError> for String {
    fn from(error: AppError) -> Self {
        error.to_string()
    }
}
