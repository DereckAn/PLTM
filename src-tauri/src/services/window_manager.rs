pub trait OverlayRenderer {
    fn init(&mut self) -> Result<()>;
    fn draw_hints(&self, hints: &[Hint]) -> Result<()>;
    fn show(&self) -> Result<()>;
    fn hide(&self) -> Result<()>;
}

#[cfg(target_os = "macos")]
type DefaultRenderer = crate::platform::macos::overlay::MacOverlayRenderer;
#[cfg(target_os = "windows")]
type DefaultRenderer = crate::platform::windows::overlay::WinOverlayRenderer;
#[cfg(target_os = "linux")]
type DefaultRenderer = crate::platform::linux::overlay::LinuxOverlayRenderer;

pub struct WindowManager<R: OverlayRenderer = DefaultRenderer> {
    renderer: R,
}

impl WindowManager {
    pub fn new() -> Result<Self> {
        let mut renderer = DefaultRenderer::new()?;
        renderer.init()?;
        Ok(Self { renderer })
    }

    /// Dibuja y muestra hints con render nativo (sin WebView).
    pub async fn show_overlay(&mut self, hints: &[Hint]) -> Result<()> {
        self.renderer.draw_hints(hints)?;
        self.renderer.show()
    }

    /// Oculta overlay y limpia buffers si aplica.
    pub async fn hide_overlay(&mut self) -> Result<()> {
        self.renderer.hide()
    }
}
