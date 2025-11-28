use crate::services::{
    AccessibilityService, ClickService, HintGenerator, HotkeyService, WindowManager,
};
use crate::Result;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct AppState {
    pub accessibility_service: AccessibilityService,
    pub hotkey_service: Arc<Mutex<HotkeyService>>,
    pub click_service: ClickService,
    #[allow(dead_code)]
    pub hint_generator: HintGenerator,
    pub window_manager: Arc<Mutex<WindowManager>>,
}

impl AppState {
    pub fn new() -> Result<Self> {
        tracing::info!("Initializing AppState...");

        let hotkeys = HotkeyService::new();
        let window_manager = WindowManager::new();

        Ok(Self {
            accessibility_service: AccessibilityService::new(),
            hotkey_service: Arc::new(Mutex::new(hotkeys)),
            click_service: ClickService::new(),
            hint_generator: HintGenerator::new(),
            window_manager: Arc::new(Mutex::new(window_manager)),
        })
    }
}
