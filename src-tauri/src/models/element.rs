#[derive(Debug, CClone, Serialize, Deserialize)]
pub struct UIElement {
    pub id: String,
    pub role: AccessibilityRole,
    pub position: Rect,
    pub title: Option<String>,
    pub value: Option<String>,
    pub is_focusable: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rect {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AccessibilityRole {
    Button,
    Link,
    TextField,
    Image,
    Checkbox,
    RadioButton,
    Slider,
    MenuItem,
    Other(String),
}
