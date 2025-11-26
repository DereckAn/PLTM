use crate::services::{
    AccessibilityService, ClickService, HintGenerator, HotkeyService, WindowManager,
};
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct AppState {
    pub accessibility_service: AccessibilityService,
    pub hotkey_service: Arc<Mutex<HotkeyService>>,
    pub click_service: ClickService,
    pub hint_generator: HintGenerator,
    pub window_manager: Arc<Mutex<WindowManager>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            accessibility_service: AccessibilityService::new(),
            hotkey_service: Arc::new(Mutex::new(HotkeyService::new())),
            click_service: ClickService::new(),
            hint_generator: HintGenerator::new(),
            window_manager: Arc::new(Mutex::new(WindowManager::new().expect("overlay"))),
        }
    }
}
