use crate::models::Hint;
use crate::Result;

/// Gestor de ventanas y overlay
/// TODO: Implementar OverlayRenderer nativo en Fase 2
pub struct WindowManager {
    is_overlay_visible: bool,
}

impl WindowManager {
    pub fn new() -> Self {
        tracing::debug!("Initializing WindowManager");
        Self {
            is_overlay_visible: false,
        }
    }

    /// Muestra el overlay con los hints proporcionados
    pub async fn show_overlay(&mut self, hints: &[Hint]) -> Result<()> {
        tracing::info!("Showing overlay with {} hints", hints.len());
        // TODO: Implementar render nativo del overlay
        self.is_overlay_visible = true;
        Ok(())
    }

    /// Oculta el overlay
    #[allow(dead_code)]
    pub async fn hide_overlay(&mut self) -> Result<()> {
        tracing::info!("Hiding overlay");
        // TODO: Implementar ocultación del overlay
        self.is_overlay_visible = false;
        Ok(())
    }

    /// Verifica si el overlay está visible
    #[allow(dead_code)]
    pub fn is_visible(&self) -> bool {
        self.is_overlay_visible
    }
}

impl Default for WindowManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_show_hide_overlay() {
        let mut wm = WindowManager::new();
        assert!(!wm.is_visible());

        wm.show_overlay(&[]).await.unwrap();
        assert!(wm.is_visible());

        wm.hide_overlay().await.unwrap();
        assert!(!wm.is_visible());
    }
}
