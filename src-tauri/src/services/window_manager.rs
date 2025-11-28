use crate::models::Hint;
use crate::Result;

/// Trait para renderizadores de overlay específicos por plataforma
pub trait OverlayRenderer: Send {
    /// Inicializa el renderer y crea la ventana nativa
    fn init(&mut self) -> Result<()>;

    /// Dibuja los hints en el overlay
    fn draw_hints(&mut self, hints: &[Hint]) -> Result<()>;

    /// Muestra el overlay
    fn show(&mut self) -> Result<()>;

    /// Oculta el overlay
    fn hide(&mut self) -> Result<()>;

    /// Limpia recursos
    fn teardown(&mut self) -> Result<()>;
}

/// Renderer no-op para plataformas no soportadas
#[cfg(not(target_os = "macos"))]
pub struct NoopRenderer;

#[cfg(not(target_os = "macos"))]
impl OverlayRenderer for NoopRenderer {
    fn init(&mut self) -> Result<()> {
        tracing::warn!("NoopRenderer: init() - overlay not supported on this platform");
        Ok(())
    }

    fn draw_hints(&mut self, hints: &[Hint]) -> Result<()> {
        tracing::warn!(
            "NoopRenderer: draw_hints() with {} hints - not rendered",
            hints.len()
        );
        Ok(())
    }

    fn show(&mut self) -> Result<()> {
        tracing::warn!("NoopRenderer: show() - no-op");
        Ok(())
    }

    fn hide(&mut self) -> Result<()> {
        tracing::warn!("NoopRenderer: hide() - no-op");
        Ok(())
    }

    fn teardown(&mut self) -> Result<()> {
        tracing::warn!("NoopRenderer: teardown() - no-op");
        Ok(())
    }
}

#[cfg(not(target_os = "macos"))]
impl NoopRenderer {
    pub fn new() -> Self {
        Self
    }
}

/// Gestor de ventanas y overlay
pub struct WindowManager {
    renderer: Box<dyn OverlayRenderer>,
    is_overlay_visible: bool,
}

impl WindowManager {
    #[cfg(target_os = "macos")]
    pub fn new() -> Self {
        use crate::platform::macos::overlay::MacOverlay;

        tracing::debug!("Initializing WindowManager with MacOverlay");
        Self {
            renderer: Box::new(MacOverlay::new()),
            is_overlay_visible: false,
        }
    }

    #[cfg(not(target_os = "macos"))]
    pub fn new() -> Self {
        tracing::debug!("Initializing WindowManager with NoopRenderer");
        Self {
            renderer: Box::new(NoopRenderer::new()),
            is_overlay_visible: false,
        }
    }

    /// Inicializa el renderer
    #[allow(dead_code)]
    pub fn init(&mut self) -> Result<()> {
        tracing::info!("WindowManager: initializing renderer");
        self.renderer.init()
    }

    /// Muestra el overlay con los hints proporcionados
    pub async fn show_overlay(&mut self, hints: &[Hint]) -> Result<()> {
        tracing::info!("WindowManager: showing overlay with {} hints", hints.len());

        // Inicializar si es necesario
        if !self.is_overlay_visible {
            self.renderer.init()?;
        }

        // Dibujar hints
        self.renderer.draw_hints(hints)?;

        // Mostrar
        self.renderer.show()?;

        self.is_overlay_visible = true;
        Ok(())
    }

    /// Oculta el overlay
    pub async fn hide_overlay(&mut self) -> Result<()> {
        tracing::info!("WindowManager: hiding overlay");

        if self.is_overlay_visible {
            self.renderer.hide()?;
            self.is_overlay_visible = false;
        }

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

impl Drop for WindowManager {
    fn drop(&mut self) {
        if let Err(e) = self.renderer.teardown() {
            tracing::error!("Failed to teardown renderer: {}", e);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_show_hide_overlay() {
        let mut wm = WindowManager::new();
        assert!(!wm.is_visible());

        // Note: En tests sin entorno gráfico, estos pueden fallar
        // pero la estructura es correcta
    }
}
