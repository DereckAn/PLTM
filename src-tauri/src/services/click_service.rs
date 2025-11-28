use crate::platform::macos::events;
use crate::Result;

pub struct ClickService;

impl ClickService {

    pub fn new() -> Self {
        Self {}
    }

    // Simula un click en las coordenadas especificas
    pub fn  perform_click(&self, x: f64, y: f64) -> Result<()> {
        // 1. Mover el cursor a (x, y)
        // 2. Simular evento de click
        events::post_mouse_click(x, y)
    }
}

