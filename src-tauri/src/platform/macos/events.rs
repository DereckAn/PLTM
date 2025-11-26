use crate::Result;
use core_graphics::event::{CGEvent, CGEventType, CGMouseButton, CGEventTapLocation};
use core_graphics::event_source::{CGEventSource, CGEventSourceStateID};
use core_graphics::geometry::CGPoint;

// Simula un click del mouse en las coordenadas especificas
pub fn post_mouse_click(x: f64, y: f64) -> Result<()> {
    let source = CGEventSource::new(CGEventSourceStateID::HIDSystemState)
        .map_err(|_| "Failed to create CGEventSource".to_string())?;
    let location = CGPoint::new(x, y);

    let mouse_down = CGEvent::new_mouse_event(
        source.clone(),
        CGEventType::LeftMouseDown,
        location,
        CGMouseButton::Left,
    )
    .map_err(|_| "Failed to create mouse down event".to_string())?;
    mouse_down.post(CGEventTapLocation::HID);

    let mouse_up = CGEvent::new_mouse_event(
        source,
        CGEventType::LeftMouseUp,
        location,
        CGMouseButton::Left,
    )
    .map_err(|_| "Failed to create mouse up event".to_string())?;
    mouse_up.post(CGEventTapLocation::HID);

    Ok(())
}
