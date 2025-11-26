use crate::models::Hint;
use crate::Result;

pub struct MacOverlayRenderer;

impl crate::services::window_manager::OverlayRenderer for MacOverlayRenderer {
    fn new() -> Result<Self> {
        Ok(Self)
    }

    fn init(&mut self) -> Result<()> {
        Ok(())
    }

    fn draw_hints(&self, _hints: &[Hint]) -> Result<()> {
        Ok(())
    }

    fn show(&self) -> Result<()> {
        Ok(())
    }

    fn hide(&self) -> Result<()> {
        Ok(())
    }
}
