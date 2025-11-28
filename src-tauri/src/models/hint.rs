use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Hint {
    pub label: String,
    pub x: f64,
    pub y: f64,
    pub element_id: String,
}

impl Hint {
    pub fn new(label: String, x: f64, y: f64, element_id: String) -> Self {
        Self {
            label,
            x,
            y,
            element_id,
        }
    }
}
