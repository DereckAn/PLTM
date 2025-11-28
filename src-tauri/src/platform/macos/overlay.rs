use objc2::rc::Retained;
use objc2_app_kit::{NSBackingStoreType, NSScreen, NSWindow, NSWindowStyleMask};
use objc2_core_foundation::{CGPoint, CGRect, CGSize};
use objc2_foundation::{MainThreadMarker, NSPoint, NSRect, NSSize, NSString};
use objc2_quartz_core::{kCAAlignmentCenter, CALayer, CATextLayer, CATransaction};

use crate::error::AppError;
use crate::models::Hint;
use crate::services::window_manager::OverlayRenderer;
use crate::Result;

const HINT_FONT_SIZE: f64 = 14.0;
const HINT_PADDING: f64 = 6.0;
const HINT_CORNER_RADIUS: f64 = 4.0;

/// Overlay nativo simple en macOS usando NSWindow + CALayer
pub struct MacOverlay {
  window: Option<Retained<NSWindow>>,
  root_layer: Option<Retained<CALayer>>,
  hint_layers: Vec<Retained<CATextLayer>>,
  scale_factor: f64,
  screen_height: f64,
  initialized: bool,
}

// SAFETY: Este renderer se usa únicamente desde el hilo principal
unsafe impl Send for MacOverlay {}

impl MacOverlay {
  pub fn new() -> Self {
    tracing::debug!("Creating MacOverlay");
    Self {
      window: None,
      root_layer: None,
      hint_layers: Vec::new(),
      scale_factor: 1.0,
      screen_height: 0.0,
      initialized: false,
    }
  }

  /// Garantiza que estamos en el hilo principal
  fn main_thread_marker(&self) -> Result<MainThreadMarker> {
    MainThreadMarker::new().ok_or_else(|| {
      AppError::Overlay("Overlay must be called from the main thread".to_string())
    })
  }

  /// Obtiene frame y escala de la pantalla principal
  fn screen_info(&self) -> Result<(NSRect, f64)> {
    let mtm = self.main_thread_marker()?;

    let info = if let Some(screen) = NSScreen::mainScreen(mtm) {
      let frame = screen.frame();
      let scale = screen.backingScaleFactor() as f64;
      (frame, scale)
    } else {
      (NSRect::new(NSPoint::new(0.0, 0.0), NSSize::new(1920.0, 1080.0)), 1.0)
    };

    Ok(info)
  }

  /// Ajusta el frame de ventana y capa raíz al tamaño de pantalla actual
  fn resize_to_screen(&mut self, frame: NSRect) -> Result<()> {
    if let Some(ref window) = self.window {
      window.setFrame_display(frame, true);
    }

    if let Some(ref root) = self.root_layer {
      let cg_frame = CGRect::new(
        CGPoint::new(0.0, 0.0),
        CGSize::new(frame.size.width as f64, frame.size.height as f64),
      );
      root.setFrame(cg_frame);
      root.setContentsScale(self.scale_factor);
    }

    Ok(())
  }

  /// Convierte coordenadas Y (origen arriba) al sistema de AppKit (origen abajo)
  fn convert_y(&self, y: f64, height: f64) -> f64 {
    self.screen_height - y - height
  }

  /// Crea la ventana transparente y la capa raíz
  fn create_window(&mut self) -> Result<()> {
    let mtm = self.main_thread_marker()?;
    let (frame, scale) = self.screen_info()?;
    self.scale_factor = scale;
    self.screen_height = frame.size.height;

    tracing::debug!(
      "Creating overlay window frame={}x{} scale={}",
      frame.size.width,
      frame.size.height,
      self.scale_factor
    );

    let window = unsafe {
      NSWindow::initWithContentRect_styleMask_backing_defer(
        mtm.alloc(),
        frame,
        NSWindowStyleMask::Borderless,
        NSBackingStoreType::Buffered,
        false,
      )
    };

    // Configurar ventana transparente y click-through
    window.setOpaque(false);
    window.setHasShadow(false);
    window.setIgnoresMouseEvents(true);

    // Configurar layer raíz
    if let Some(content_view) = window.contentView() {
      content_view.setWantsLayer(true);

      let root_layer = CALayer::new();
      let cg_frame = CGRect::new(
        CGPoint::new(0.0, 0.0),
        CGSize::new(frame.size.width as f64, frame.size.height as f64),
      );

      root_layer.setFrame(cg_frame);
      root_layer.setContentsScale(self.scale_factor);
      content_view.setLayer(Some(&root_layer));

      self.root_layer = Some(root_layer);
    }

    self.window = Some(window);
    self.resize_to_screen(frame)?;

    Ok(())
  }

  /// Crea un CATextLayer para un hint
  fn create_hint_layer(&self, hint: &Hint) -> Result<Retained<CATextLayer>> {
    let layer = CATextLayer::new();

    let text = NSString::from_str(&hint.label);

    // Tamaños
    let text_width = hint.label.len() as f64 * HINT_FONT_SIZE * 0.8;
    let layer_width = (text_width + HINT_PADDING * 2.5).max(HINT_FONT_SIZE * 2.0);
    let layer_height = HINT_FONT_SIZE + HINT_PADDING * 2.5;

    // Coordenadas
    let x = hint.x - layer_width / 2.0;
    let y = self.convert_y(hint.y, layer_height);

    let frame = CGRect::new(
      CGPoint::new(x, y),
      CGSize::new(layer_width, layer_height),
    );

    // SAFETY: Configuración del layer en hilo principal
    unsafe {
      layer.setFrame(frame);
      layer.setCornerRadius(HINT_CORNER_RADIUS);
      layer.setString(Some(&text));
      layer.setFontSize(HINT_FONT_SIZE);
      layer.setAlignmentMode(kCAAlignmentCenter);
      layer.setContentsScale(self.scale_factor);
    }

    Ok(layer)
  }

  fn clear_hint_layers(&mut self) {
    if self.hint_layers.is_empty() {
      return;
    }

    CATransaction::begin();
    CATransaction::setDisableActions(true);
    for layer in &self.hint_layers {
      layer.removeFromSuperlayer();
    }
    CATransaction::commit();

    self.hint_layers.clear();
  }
}

impl OverlayRenderer for MacOverlay {
  fn init(&mut self) -> Result<()> {
    if self.initialized {
      return Ok(());
    }

    self.main_thread_marker()?;
    self.create_window()?;
    self.initialized = true;
    Ok(())
  }

  fn draw_hints(&mut self, hints: &[Hint]) -> Result<()> {
    if !self.initialized {
      return Err(AppError::Overlay("Overlay not initialized".to_string()));
    }

    let (frame, scale) = self.screen_info()?;
    self.scale_factor = scale;
    self.screen_height = frame.size.height;
    self.resize_to_screen(frame)?;

    self.clear_hint_layers();

    let Some(root) = self.root_layer.clone() else {
      return Err(AppError::Overlay("Root layer not available".to_string()));
    };

    CATransaction::begin();
    CATransaction::setDisableActions(true);

    for hint in hints {
      let hint_layer = self.create_hint_layer(hint)?;
      root.addSublayer(&hint_layer);
      self.hint_layers.push(hint_layer);
    }

    CATransaction::commit();

    Ok(())
  }

  fn show(&mut self) -> Result<()> {
    if let Some(ref window) = self.window {
      window.makeKeyAndOrderFront(None);
    }
    Ok(())
  }

  fn hide(&mut self) -> Result<()> {
    self.clear_hint_layers();

    if let Some(ref window) = self.window {
      window.orderOut(None);
    }
    Ok(())
  }

  fn teardown(&mut self) -> Result<()> {
    self.clear_hint_layers();

    if let Some(ref window) = self.window {
      window.orderOut(None);
      window.close();
    }

    self.window = None;
    self.root_layer = None;
    self.initialized = false;
    Ok(())
  }
}

impl Default for MacOverlay {
  fn default() -> Self {
    Self::new()
  }
}
