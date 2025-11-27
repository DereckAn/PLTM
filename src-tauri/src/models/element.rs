use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UIElement {
    pub id: String,
    pub role: String,
    pub title: Option<String>,
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

impl UIElement {
    pub fn new(
        id: String,
        role: String,
        title: Option<String>,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
    ) -> Self {
        Self {
            id,
            role,
            title,
            x,
            y,
            width,
            height,
        }
    }
}
