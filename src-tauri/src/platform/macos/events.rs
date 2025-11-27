use crate::{AppError, Result};
use core_graphics::event::{CGEvent, CGEventTapLocation, CGEventType, CGMouseButton};
use core_graphics::event_source::{CGEventSource, CGEventSourceStateID};
use core_graphics::geometry::CGPoint;

fn create_mouse_event(
    source: CGEventSource,
    event_type: CGEventType,
    location: CGPoint,
    button: CGMouseButton,
    error_msg: &str,
) -> Result<CGEvent> {
    CGEvent::new_mouse_event(source, event_type, location, button).map_err(|_| {
        tracing::error!("{}", error_msg);
        AppError::Click(error_msg.to_string())
    })
}

pub fn post_mouse_click(x: f64, y: f64) -> Result<()> {
    tracing::info!("Simulating mouse click at ({}, {})", x, y);

    let source = CGEventSource::new(CGEventSourceStateID::HIDSystemState).map_err(|_| {
        tracing::error!("Failed to create CGEventSource");
        AppError::Click("Failed to create CGEventSource".to_string())
    })?;

    let location = CGPoint::new(x, y);

    let mouse_down = create_mouse_event(
        source.clone(),
        CGEventType::LeftMouseDown,
        location,
        CGMouseButton::Left,
        "Failed to create mouse down event",
    )?;
    mouse_down.post(CGEventTapLocation::HID);

    let mouse_up = create_mouse_event(
        source,
        CGEventType::LeftMouseUp,
        location,
        CGMouseButton::Left,
        "Failed to create mouse up event",
    )?;
    mouse_up.post(CGEventTapLocation::HID);

    tracing::info!("Mouse click simulated successfully at ({}, {})", x, y);
    Ok(())
}
