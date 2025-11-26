use core_graphics::event::{CGEvent, CGEventType};
use core_graphics::event_source::{CGEventSource, CGEventSourceStateID};

// Simula un click del mouse en las coordenadas especificas
pub fn post_mouse_click(x: f64, y: f64) -> Result<()> {
    unsafe {
        let source = CGEventSource::new(CGEventSourceStateID::HIDSystemState)?;
        let location = CGPoint::new(x, y);

        // Mouse down
        let mouse_down = CGEvent::new_mouse_event(
            source.clone(),
            CGEventType::LeftMouseDown,
            location,
            CGMouseButton::Left,
        )?;
        mouse_down.post(CGEventTapLocation::HID);

        // Mouse up
        let mouse_up = CGEvent::new_mouse_event(
            source,
            CGEventType::LeftMouseUp,
            location,
            CGMouseButton::Left,
        )?;
        mouse_up.post(CGEventTapLocation::HID)?;

        Ok(())
    }
}