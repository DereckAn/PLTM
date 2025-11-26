# Plan de Arquitectura Enterprise
## Aplicación de Navegación por Teclado (Homerow-like)

### Stack Tecnológico
- **Frontend**: Svelte 5 + TailwindCSS + TypeScript
- **Backend/Core**: Rust + Tauri 2.x
- **Overlay Renderer**: Nativo por OS (macOS CoreGraphics/Metal; Windows Direct2D/DirectWrite; Linux Cairo/Pango/Wayland surfaces)
- **Package Manager**: Bun
- **Testing**: Vitest + Rust testing framework
- **CI/CD**: GitHub Actions

---

## 1. Arquitectura General

```
┌─────────────────────────────────────────────────────────────┐
│                     PRESENTATION LAYER                       │
│  ┌────────────────────────────────────────────────────────┐ │
│  │  Native Overlay Renderer (per-OS) + Svelte Settings     │ │
│  │  • HintLayer (CG/Metal/D2D/Cairo) • SettingsPanel     │ │
│  └────────────────────────────────────────────────────────┘ │
└─────────────────────────────────────────────────────────────┘
                              ▼
┌─────────────────────────────────────────────────────────────┐
│                    APPLICATION LAYER                         │
│  ┌────────────────────────────────────────────────────────┐ │
│  │              Tauri Command Interface                    │ │
│  │  • IPC Bridge  • State Management  • Event System     │ │
│  └────────────────────────────────────────────────────────┘ │
└─────────────────────────────────────────────────────────────┘
                              ▼
┌─────────────────────────────────────────────────────────────┐
│                      BUSINESS LOGIC LAYER                    │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────────┐  │
│  │  Accessibility│  │   Hotkey     │  │   Click          │  │
│  │    Service    │  │   Service    │  │   Service        │  │
│  └──────────────┘  └──────────────┘  └──────────────────┘  │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────────┐  │
│  │   Hint       │  │   Window     │  │   Configuration  │  │
│  │   Generator  │  │   Manager    │  │   Manager        │  │
│  └──────────────┘  └──────────────┘  └──────────────────┘  │
└─────────────────────────────────────────────────────────────┘
                              ▼
┌─────────────────────────────────────────────────────────────┐
│               PLATFORM LAYER (macOS/Win/Linux)              │
│  ┌────────────────────────────────────────────────────────┐ │
│  │        Native FFI Bindings (Rust por OS)                │ │
│  │  • macOS: CoreGraphics/CoreAnimation/AX                 │ │
│  │  • Windows: Direct2D/DirectWrite/UIA                    │ │
│  │  • Linux: Cairo/Pango + AT-SPI2 (X11/Wayland)           │ │
│  └────────────────────────────────────────────────────────┘ │
└─────────────────────────────────────────────────────────────┘
```

---

## 2. Estructura de Proyecto

```
keyboard-nav-app/
├── src/                                    # Frontend (Svelte)
│   ├── lib/
│   │   ├── components/
│   │   │   ├── settings/
│   │   │   │   ├── SettingsPanel.svelte
│   │   │   │   ├── HotkeyConfig.svelte
│   │   │   │   └── AppearanceConfig.svelte
│   │   │   └── permissions/
│   │   │       └── PermissionGuard.svelte
│   │   ├── stores/
│   │   │   ├── hints.ts
│   │   │   ├── settings.ts
│   │   │   └── app-state.ts
│   │   ├── services/
│   │   │   ├── tauri-commands.ts
│   │   │   ├── event-listener.ts
│   │   │   └── keyboard-handler.ts
│   │   └── utils/
│   │       ├── hint-generator.ts
│   │       └── geometry.ts
│   ├── routes/
│   │   ├── +layout.svelte
│   │   └── +page.svelte
│   └── app.css
│
├── src-tauri/                              # Backend (Rust)
│   ├── src/
│   │   ├── main.rs
│   │   ├── lib.rs
│   │   ├── commands/                       # Tauri Commands
│   │   │   ├── mod.rs
│   │   │   ├── accessibility.rs
│   │   │   ├── hotkey.rs
│   │   │   └── window.rs
│   │   ├── services/                       # Business Logic
│   │   │   ├── mod.rs
│   │   │   ├── accessibility_service.rs
│   │   │   ├── hotkey_service.rs
│   │   │   ├── click_service.rs
│   │   │   ├── hint_generator.rs
│   │   │   └── window_manager.rs
│   │   ├── models/                         # Data Models
│   │   │   ├── mod.rs
│   │   │   ├── element.rs
│   │   │   ├── hint.rs
│   │   │   └── config.rs
│   │   ├── platform/                       # Platform-specific
│   │   │   ├── mod.rs
│   │   │   ├── macos/
│   │   │   │   ├── mod.rs
│   │   │   │   ├── accessibility.rs
│   │   │   │   ├── window.rs
│   │   │   │   ├── overlay.rs              # Render nativo (CG/Metal)
│   │   │   │   └── events.rs
│   │   │   ├── windows/                    # Future support
│   │   │   │   ├── mod.rs
│   │   │   │   └── overlay.rs              # Render nativo (Direct2D/DirectWrite)
│   │   │   └── linux/                      # Future support
│   │   │       ├── mod.rs
│   │   │       └── overlay.rs              # Render nativo (Cairo/Pango/Wayland)
│   │   ├── state/                          # Global State
│   │   │   ├── mod.rs
│   │   │   └── app_state.rs
│   │   ├── error/                          # Error Handling
│   │   │   ├── mod.rs
│   │   │   └── app_error.rs
│   │   └── utils/
│   │       ├── mod.rs
│   │       └── logger.rs
│   ├── Cargo.toml
│   └── tauri.conf.json
│
├── tests/
│   ├── e2e/
│   └── integration/
├── .github/
│   └── workflows/
│       ├── ci.yml
│       └── release.yml
├── bun.lockb
├── package.json
├── tailwind.config.js
├── svelte.config.js
└── README.md
```

---

## 3. Componentes Core (Rust)

### 3.1 Accessibility Service
```rust
// src-tauri/src/services/accessibility_service.rs

use crate::models::element::UIElement;
use crate::platform::macos::accessibility;

pub struct AccessibilityService {
    enabled: bool,
    cache: ElementCache,
}

impl AccessibilityService {
    pub fn new() -> Self { /* ... */ }
    
    /// Escanea todos los elementos clickeables en la pantalla
    pub async fn scan_clickable_elements(&self) -> Result<Vec<UIElement>> {
        // 1. Verificar permisos
        // 2. Obtener ventana activa
        // 3. Recorrer árbol de accesibilidad
        // 4. Filtrar elementos clickeables
        // 5. Calcular posiciones en pantalla
    }
    
    /// Verifica si tenemos permisos de accesibilidad
    pub fn check_permissions(&self) -> bool {
        accessibility::has_accessibility_permissions()
    }
    
    /// Solicita permisos al usuario
    pub fn request_permissions(&self) -> Result<()> {
        accessibility::request_permissions()
    }
}
```

### 3.2 Hotkey Service
```rust
// src-tauri/src/services/hotkey_service.rs

use global_hotkey::{GlobalHotKeyManager, HotKey};
use tauri::Manager;

pub struct HotkeyService {
    manager: GlobalHotKeyManager,
    registered_hotkeys: HashMap<String, HotKey>,
}

impl HotkeyService {
    pub fn new() -> Self { /* ... */ }
    
    /// Registra el hotkey principal para activar la navegación
    pub fn register_activation_hotkey(&mut self, key_combo: &str) -> Result<()> {
        // Ej: "Cmd+J"
    }
    
    /// Maneja el evento cuando se presiona el hotkey
    pub async fn handle_hotkey_pressed(&self, app: &AppHandle) {
        // 1. Activar modo navegación
        // 2. Escanear elementos
        // 3. Generar hints
        // 4. Mostrar overlay
    }
}
```

### 3.3 Hint Generator
```rust
// src-tauri/src/services/hint_generator.rs

pub struct HintGenerator {
    sequence: HintSequence,
}

impl HintGenerator {
    /// Genera secuencias de hints (ej: "a", "b", "aa", "ab"...)
    pub fn generate_hints(&self, count: usize) -> Vec<String> {
        // Algoritmo para generar hints eficientes
        // Similar a Vimium: usa caracteres del home row
    }
    
    /// Estrategia de generación: home row priority
    fn get_hint_chars(&self) -> Vec<char> {
        vec!['a', 's', 'd', 'f', 'j', 'k', 'l', 'h', 'g']
    }
}
```

### 3.4 Click Service
```rust
// src-tauri/src/services/click_service.rs

use crate::platform::macos::events;

pub struct ClickService;

impl ClickService {
    /// Simula un click en las coordenadas especificadas
    pub fn perform_click(&self, x: f64, y: f64) -> Result<()> {
        events::post_mouse_click(x, y)
    }
    
    /// Simula un click con modificadores (Cmd, Shift, etc)
    pub fn perform_click_with_modifiers(
        &self, 
        x: f64, 
        y: f64,
        modifiers: KeyModifiers
    ) -> Result<()> {
        // Para abrir en nueva pestaña, etc.
    }
}
```

### 3.5 Window Manager (Render nativo por OS)
```rust
// src-tauri/src/services/window_manager.rs

pub trait OverlayRenderer {
    fn init(&mut self) -> Result<()>;
    fn draw_hints(&self, hints: &[Hint]) -> Result<()>;
    fn show(&self) -> Result<()>;
    fn hide(&self) -> Result<()>;
}

#[cfg(target_os = "macos")]
type DefaultRenderer = crate::platform::macos::overlay::MacOverlayRenderer;
#[cfg(target_os = "windows")]
type DefaultRenderer = crate::platform::windows::overlay::WinOverlayRenderer;
#[cfg(target_os = "linux")]
type DefaultRenderer = crate::platform::linux::overlay::LinuxOverlayRenderer;

pub struct WindowManager<R: OverlayRenderer = DefaultRenderer> {
    renderer: R,
}

impl WindowManager {
    pub fn new() -> Result<Self> {
        let mut renderer = DefaultRenderer::new()?;
        renderer.init()?;
        Ok(Self { renderer })
    }

    /// Dibuja y muestra hints con render nativo (sin WebView).
    pub async fn show_overlay(&mut self, hints: &[Hint]) -> Result<()> {
        self.renderer.draw_hints(hints)?;
        self.renderer.show()
    }

    /// Oculta overlay y limpia buffers si aplica.
    pub async fn hide_overlay(&mut self) -> Result<()> {
        self.renderer.hide()
    }
}
```

---

## 4. Models (Rust)

```rust
// src-tauri/src/models/element.rs

#[derive(Debug, Clone, Serialize, Deserialize)]
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
    Checkbox,
    MenuItem,
    Other(String),
}
```

```rust
// src-tauri/src/models/hint.rs

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Hint {
    pub id: String,
    pub label: String,
    pub element: UIElement,
    pub position: Position,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position {
    pub x: f64,
    pub y: f64,
}
```

---

## 5. Platform Layer (macOS)

```rust
// src-tauri/src/platform/macos/accessibility.rs

use cocoa::base::{id, nil};
use cocoa::foundation::NSString;
use core_foundation::*;

/// Verifica si tenemos permisos de accesibilidad
pub fn has_accessibility_permissions() -> bool {
    unsafe {
        let trusted = AXIsProcessTrustedWithOptions(nil);
        trusted != 0
    }
}

/// Solicita permisos abriendo Preferencias del Sistema
pub fn request_permissions() -> Result<()> {
    unsafe {
        let options = /* crear diccionario con kAXTrustedCheckOptionPrompt */;
        AXIsProcessTrustedWithOptions(options);
    }
    Ok(())
}

/// Obtiene la ventana activa del sistema
pub fn get_active_window() -> Result<id> {
    unsafe {
        let system_wide = AXUIElementCreateSystemWide();
        let mut focused_app: id = nil;
        
        AXUIElementCopyAttributeValue(
            system_wide,
            kAXFocusedApplicationAttribute,
            &mut focused_app as *mut _ as *mut _
        );
        
        Ok(focused_app)
    }
}

/// Recorre el árbol de accesibilidad recursivamente
pub fn traverse_accessibility_tree(element: id) -> Vec<id> {
    // Implementación recursiva para obtener todos los elementos
}
```

```rust
// src-tauri/src/platform/macos/events.rs

use core_graphics::event::{CGEvent, CGEventType};
use core_graphics::event_source::{CGEventSource, CGEventSourceStateID};

/// Simula un click del mouse en las coordenadas especificadas
pub fn post_mouse_click(x: f64, y: f64) -> Result<()> {
    unsafe {
        let source = CGEventSource::new(CGEventSourceStateID::HIDSystemState)?;
        let location = CGPoint::new(x, y);
        
        // Mouse down
        let mouse_down = CGEvent::new_mouse_event(
            source.clone(),
            CGEventType::LeftMouseDown,
            location,
            CGMouseButton::Left
        )?;
        mouse_down.post(CGEventTapLocation::HID);
        
        // Mouse up
        let mouse_up = CGEvent::new_mouse_event(
            source,
            CGEventType::LeftMouseUp,
            location,
            CGMouseButton::Left
        )?;
        mouse_up.post(CGEventTapLocation::HID);
        
        Ok(())
    }
}
```

---

## 6. Tauri Commands

```rust
// src-tauri/src/commands/accessibility.rs

#[tauri::command]
pub async fn scan_elements(
    state: State<'_, AppState>
) -> Result<Vec<UIElement>, String> {
    state
        .accessibility_service
        .scan_clickable_elements()
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn check_permissions(
    state: State<'_, AppState>
) -> Result<bool, String> {
    Ok(state.accessibility_service.check_permissions())
}

#[tauri::command]
pub async fn request_permissions(
    state: State<'_, AppState>
) -> Result<(), String> {
    state
        .accessibility_service
        .request_permissions()
        .map_err(|e| e.to_string())
}
```

```rust
// src-tauri/src/commands/hotkey.rs

#[tauri::command]
pub async fn register_hotkey(
    key_combo: String,
    state: State<'_, AppState>
) -> Result<(), String> {
    state
        .hotkey_service
        .lock()
        .await
        .register_activation_hotkey(&key_combo)
        .map_err(|e| e.to_string())
}
```

```rust
// src-tauri/src/commands/window.rs

#[tauri::command]
pub async fn show_hints(
    hints: Vec<Hint>,
    app: AppHandle,
    state: State<'_, AppState>
) -> Result<(), String> {
    state
        .window_manager
        .lock()
        .await
        .show_overlay(hints)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn perform_click(
    x: f64,
    y: f64,
    state: State<'_, AppState>
) -> Result<(), String> {
    state
        .click_service
        .perform_click(x, y)
        .map_err(|e| e.to_string())
}
```

---

## 7. Frontend (Svelte)

### 7.1 Stores

```typescript
// src/lib/stores/hints.ts

import { writable, derived } from 'svelte/store';
import type { Hint } from '$lib/types';

export const hints = writable<Hint[]>([]);
export const activeHint = writable<string>('');
export const inputSequence = writable<string>('');

export const filteredHints = derived(
  [hints, inputSequence],
  ([$hints, $inputSequence]) => {
    if (!$inputSequence) return $hints;
    return $hints.filter(h => 
      h.label.startsWith($inputSequence.toLowerCase())
    );
  }
);
```

```typescript
// src/lib/stores/app-state.ts

import { writable } from 'svelte/store';

export type AppMode = 'idle' | 'navigation' | 'settings';

export const appMode = writable<AppMode>('idle');
export const hasPermissions = writable<boolean>(false);
export const isLoading = writable<boolean>(false);
```

### 7.2 Tauri Commands Wrapper

```typescript
// src/lib/services/tauri-commands.ts

import { invoke } from '@tauri-apps/api/core';
import type { UIElement, Hint } from '$lib/types';

export class TauriCommands {
  static async scanElements(): Promise<UIElement[]> {
    return invoke('scan_elements');
  }
  
  static async checkPermissions(): Promise<boolean> {
    return invoke('check_permissions');
  }
  
  static async requestPermissions(): Promise<void> {
    return invoke('request_permissions');
  }
  
  static async registerHotkey(keyCombo: string): Promise<void> {
    return invoke('register_hotkey', { keyCombo });
  }
  
  static async performClick(x: number, y: number): Promise<void> {
    return invoke('perform_click', { x, y });
  }
  
  static async showHints(hints: Hint[]): Promise<void> {
    return invoke('show_hints', { hints });
  }
}
```

### 7.3 Components

> El overlay de hints ahora se renderiza nativamente (CoreGraphics/Metal, Direct2D, Cairo/Wayland). Svelte se usa solo para paneles de configuración y permisos.

```svelte
<!-- src/lib/components/settings/SettingsPanel.svelte -->

<script lang="ts">
  import { hasPermissions } from '$lib/stores/app-state';
  import { TauriCommands } from '$lib/services/tauri-commands';
  import { onMount } from 'svelte';

  let activate = 'Cmd+J';
  let scanDepth = 8;
  let maxHints = 400;

  onMount(async () => {
    hasPermissions.set(await TauriCommands.checkPermissions());
  });

  async function requestAccess() {
    await TauriCommands.requestPermissions();
    hasPermissions.set(await TauriCommands.checkPermissions());
  }

  async function save() {
    await TauriCommands.registerHotkey(activate);
    // TODO: Persist config via Tauri command
  }
</script>

<section class="space-y-4">
  <header class="flex items-center gap-3">
    <h1 class="text-xl font-semibold">PLTM</h1>
    {#if $hasPermissions}
      <span class="px-2 py-1 rounded bg-green-100 text-green-700 text-xs">Permisos OK</span>
    {:else}
      <button class="px-2 py-1 text-xs rounded bg-amber-100 text-amber-800" on:click={requestAccess}>
        Otorgar permisos
      </button>
    {/if}
  </header>

  <label class="flex items-center gap-2">
    <span class="w-36 text-sm font-medium">Hotkey principal</span>
    <input bind:value={activate} class="border px-2 py-1 rounded w-32" />
  </label>

  <label class="flex items-center gap-2">
    <span class="w-36 text-sm font-medium">Profundidad scan</span>
    <input type="number" min="1" max="15" bind:value={scanDepth} class="border px-2 py-1 rounded w-20" />
  </label>

  <label class="flex items-center gap-2">
    <span class="w-36 text-sm font-medium">Máx. hints</span>
    <input type="number" min="50" max="1000" bind:value={maxHints} class="border px-2 py-1 rounded w-24" />
  </label>

  <button class="px-3 py-2 rounded bg-blue-600 text-white" on:click={save}>
    Guardar
  </button>
</section>
```

---

## 8. Estado Global (Rust)

```rust
// src-tauri/src/state/app_state.rs

use crate::services::*;

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
            click_service: ClickService,
            hint_generator: HintGenerator::new(),
            window_manager: Arc::new(Mutex::new(WindowManager::new())),
        }
    }
}
```

---

## 9. Error Handling

```rust
// src-tauri/src/error/app_error.rs

use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("No accessibility permissions")]
    NoPermissions,
    
    #[error("Failed to scan elements: {0}")]
    ScanFailed(String),
    
    #[error("Failed to register hotkey: {0}")]
    HotkeyRegistrationFailed(String),
    
    #[error("Window error: {0}")]
    WindowError(String),
    
    #[error("Platform error: {0}")]
    PlatformError(String),
}

impl From<AppError> for String {
    fn from(error: AppError) -> Self {
        error.to_string()
    }
}
```

---

## 10. Configuración

### Cargo.toml
```toml
[dependencies]
tauri = { version = "2.0", features = ["devtools"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
tokio = { version = "1", features = ["full"] }
anyhow = "1.0"
thiserror = "1.0"
global-hotkey = "0.6"
log = "0.4"
env_logger = "0.11"

[target.'cfg(target_os = "macos")'.dependencies]
cocoa = "0.25"
core-foundation = "0.9"
core-graphics = "0.23"
objc = "0.2"
```

### tauri.conf.json
```json
{
  "productName": "KeyboardNav",
  "identifier": "com.yourcompany.keyboardnav",
  "build": {
    "beforeDevCommand": "bun run dev",
    "beforeBuildCommand": "bun run build",
    "devUrl": "http://localhost:5173",
    "frontendDist": "../dist"
  },
  "app": {
    "security": {
      "csp": null
    },
    "windows": [
      {
        "title": "KeyboardNav",
        "width": 0,
        "height": 0,
        "visible": false,
        "transparent": true,
        "decorations": false,
        "alwaysOnTop": true
      }
    ]
  }
}
```

---

## 11. Testing Strategy

### Unit Tests (Rust)
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_hint_generation() {
        let generator = HintGenerator::new();
        let hints = generator.generate_hints(10);
        assert_eq!(hints.len(), 10);
        assert!(hints[0].len() <= 2);
    }
    
    #[tokio::test]
    async fn test_accessibility_service() {
        let service = AccessibilityService::new();
        let has_perms = service.check_permissions();
        assert!(has_perms || !has_perms); // Should not panic
    }
}
```

### Integration Tests (Rust)
```rust
// tests/integration/scan_test.rs

#[tokio::test]
async fn test_full_scan_workflow() {
    let state = AppState::new();
    
    if !state.accessibility_service.check_permissions() {
        println!("Skipping test - no permissions");
        return;
    }
    
    let elements = state.accessibility_service
        .scan_clickable_elements()
        .await
        .unwrap();
    
    assert!(!elements.is_empty());
}
```

### E2E Tests (Playwright + Svelte)
```typescript
// tests/e2e/navigation.spec.ts

import { test, expect } from '@playwright/test';

test('should show hints on activation', async ({ page }) => {
  await page.goto('http://localhost:5173');
  
  // Simular hotkey (esto requiere configuración especial)
  await page.keyboard.press('Meta+J');
  
  // Verificar que aparecen los hints
  await expect(page.locator('[data-hint]')).toBeVisible();
});
```

---

## 12. CI/CD Pipeline

```yaml
# .github/workflows/ci.yml

name: CI

on: [push, pull_request]

jobs:
  test:
    runs-on: macos-latest
    steps:
      - uses: actions/checkout@v4
      
      - name: Setup Bun
        uses: oven-sh/setup-bun@v1
        
      - name: Setup Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          
      - name: Install dependencies
        run: bun install
        
      - name: Run frontend tests
        run: bun test
        
      - name: Run Rust tests
        working-directory: src-tauri
        run: cargo test
        
      - name: Build
        run: bun run tauri build
        
  lint:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: oven-sh/setup-bun@v1
      
      - name: Run linters
        run: |
          bun run lint
          bun run format:check
          
      - name: Rust fmt check
        working-directory: src-tauri
        run: cargo fmt --check
```

---

## 13. Consideraciones de Seguridad

### Permisos macOS
```rust
// Solicitar en Info.plist
<key>NSAppleEventsUsageDescription</key>
<string>This app needs accessibility access to enable keyboard navigation</string>
```

### Validación de Input
```rust
pub fn validate_coordinates(x: f64, y: f64) -> Result<()> {
    if x < 0.0 || y < 0.0 {
        return Err(AppError::InvalidCoordinates);
    }
    Ok(())
}
```

---

## 14. Performance Optimizations

1. **Spatial indexing (crítico)**: R-tree (`rstar`) para queries O(log n) de elementos visibles y detección rápida de overlaps.
2. **SIMD**: Filtrado masivo de coordenadas con `std::simd` para 4-8x throughput en checks de bounds/visibilidad.
3. **Memory pool**: Reusar `UIElement`/`Hint` en hot path para evitar allocs y churn del heap.
4. **Pipeline async sin bloqueos**: Escaneo, generación de hints y render en stages paralelos; comunicación por canales lock-free.
5. **Render GPU-first**: Metal (macOS), Direct2D/DirectComposition (Win), Cairo/Skia/Vulkan (Linux) con atlas de glyphs y batch draw.
6. **Profiles release agresivos**: `lto = "fat"`, `codegen-units = 1`, `panic = "abort"`, feature flags `optimized`/`debug`.

---

## 15. Roadmap de Implementación

### Fase 1: Foundation (2-3 semanas)
- [ ] Setup proyecto Tauri + Svelte + Bun
- [ ] Configurar TailwindCSS y estructura base
- [ ] Implementar FFI bindings básicos para macOS Accessibility API
- [ ] Crear sistema de permisos y guards
- [ ] Implementar logging y error handling base

### Fase 2: Core Functionality (3-4 semanas)
- [ ] Implementar AccessibilityService completo
- [ ] Sistema de scanning de elementos UI
- [ ] HotkeyService con registro global de atajos
- [ ] HintGenerator con algoritmo eficiente
- [ ] WindowManager + renderer nativo por OS (sin WebView)
- [ ] ClickService para simulación de clicks
- [ ] Spatial index (R-tree) + filtro SIMD + memory pool en hot path
- [ ] Pipeline async sin bloqueos (scan -> hints -> render)

### Fase 3: UI/UX (2-3 semanas)
- [ ] Componentes Svelte para settings/permisos (overlay nativo)
- [ ] Sistema de hints visuales con animaciones en renderer nativo
- [ ] Panel de configuración
- [ ] Temas y personalización visual
- [ ] Feedback visual (highlights, efectos)

### Fase 4: Advanced Features (2-3 semanas)
- [ ] Filtrado inteligente de elementos
- [ ] Soporte para scroll automático
- [ ] Múltiples modos de navegación (links only, buttons only, etc)
- [ ] Custom hotkeys configurables
- [ ] Blacklist/whitelist de aplicaciones

### Fase 5: Polish & Testing (2-3 semanas)
- [ ] Tests unitarios completos
- [ ] Tests de integración
- [ ] E2E testing
- [ ] Optimización de performance
- [ ] Documentación completa
- [ ] Beta testing

### Fase 6: Release (1-2 semanas)
- [ ] CI/CD pipeline
- [ ] Auto-updates con Tauri updater
- [ ] Signing para distribución macOS
- [ ] App Store / DMG distribution
- [ ] Marketing website

---

## 16. Detalles de Implementación Críticos

### 16.1 Accessibility Tree Traversal

```rust
// src-tauri/src/platform/macos/accessibility.rs

use core_foundation::array::CFArray;
use core_foundation::base::TCFType;

pub fn traverse_element(
    element: AXUIElementRef,
    depth: usize,
    max_depth: usize,
    results: &mut Vec<AccessibleElement>
) -> Result<()> {
    if depth > max_depth {
        return Ok(());
    }
    
    // Obtener rol del elemento
    let role = get_element_role(element)?;
    
    // Filtrar elementos relevantes
    if is_clickable_role(&role) {
        let position = get_element_position(element)?;
        let size = get_element_size(element)?;
        let title = get_element_title(element);
        
        results.push(AccessibleElement {
            role,
            position,
            size,
            title,
            element: element.clone(),
        });
    }
    
    // Obtener hijos y recursionar
    let children = get_element_children(element)?;
    for child in children.iter() {
        traverse_element(child, depth + 1, max_depth, results)?;
    }
    
    Ok(())
}

fn is_clickable_role(role: &str) -> bool {
    matches!(
        role,
        "AXButton" | "AXLink" | "AXMenuItem" | 
        "AXCheckBox" | "AXRadioButton" | "AXPopUpButton" |
        "AXTextField" | "AXTextArea" | "AXSearchField"
    )
}

fn get_element_position(element: AXUIElementRef) -> Result<CGPoint> {
    unsafe {
        let mut position_value: CFTypeRef = ptr::null();
        let result = AXUIElementCopyAttributeValue(
            element,
            kAXPositionAttribute,
            &mut position_value
        );
        
        if result == kAXErrorSuccess {
            let mut point = CGPoint::new(0.0, 0.0);
            AXValueGetValue(
                position_value as AXValueRef,
                kAXValueCGPointType,
                &mut point as *mut _ as *mut c_void
            );
            Ok(point)
        } else {
            Err(AppError::AccessibilityError("Failed to get position".into()))
        }
    }
}
```

### 16.2 Hint Generation Algorithm

```rust
// src-tauri/src/services/hint_generator.rs

pub struct HintGenerator {
    charset: Vec<char>,
}

impl HintGenerator {
    pub fn new() -> Self {
        Self {
            // Home row keys para máxima eficiencia
            charset: vec!['a', 's', 'd', 'f', 'j', 'k', 'l', 'h'],
        }
    }
    
    pub fn generate_hints(&self, count: usize) -> Vec<String> {
        if count == 0 {
            return vec![];
        }
        
        let base = self.charset.len();
        let mut hints = Vec::with_capacity(count);
        
        // Calcular cuántos dígitos necesitamos
        let digits_needed = ((count as f64).log(base as f64).ceil() as usize).max(1);
        
        for i in 0..count {
            let hint = self.number_to_hint(i, base, digits_needed);
            hints.push(hint);
        }
        
        hints
    }
    
    fn number_to_hint(&self, mut num: usize, base: usize, min_length: usize) -> String {
        let mut hint = String::new();
        
        loop {
            let digit = num % base;
            hint.insert(0, self.charset[digit]);
            num /= base;
            
            if num == 0 && hint.len() >= min_length {
                break;
            }
        }
        
        // Padding si es necesario
        while hint.len() < min_length {
            hint.insert(0, self.charset[0]);
        }
        
        hint
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_hint_generation() {
        let gen = HintGenerator::new();
        
        let hints = gen.generate_hints(10);
        assert_eq!(hints.len(), 10);
        
        // Verificar que todos son únicos
        let unique: std::collections::HashSet<_> = hints.iter().collect();
        assert_eq!(unique.len(), hints.len());
        
        // Verificar longitud razonable
        assert!(hints.iter().all(|h| h.len() <= 2));
    }
    
    #[test]
    fn test_large_number_of_hints() {
        let gen = HintGenerator::new();
        let hints = gen.generate_hints(100);
        
        assert_eq!(hints.len(), 100);
        assert!(hints.iter().all(|h| h.len() <= 3));
    }
}
```

### 16.3 Overlay Window Configuration (render nativo)

```rust
// src-tauri/src/platform/macos/overlay.rs (similar para win/linux)

pub struct MacOverlayRenderer {
    window: StrongPtr,          // NSWindow transparente, fullscreen
    text_layer: StrongPtr,      // CALayer que contiene CATextLayers
    glyph_cache: GlyphAtlas,    // Cache de glyphs para reusar shapes
}

impl MacOverlayRenderer {
    pub fn new() -> Result<Self> { /* crear window borderless + ignoresMouseEvents */ }

    pub fn init(&mut self) -> Result<()> {
        unsafe {
            let ns_window: id = self.window.as_ref();
            ns_window.setLevel_(NSWindowLevel::NSTornOffMenuWindowLevel);
            ns_window.setOpaque_(NO);
            ns_window.setBackgroundColor_(NSColor::clearColor());
            ns_window.setIgnoresMouseEvents_(YES);
            ns_window.setCollectionBehavior_(
                NSWindowCollectionBehaviorCanJoinAllSpaces
                    | NSWindowCollectionBehaviorStationary
                    | NSWindowCollectionBehaviorIgnoresCycle,
            );
        }
        Ok(())
    }

    pub fn draw_hints(&self, hints: &[Hint]) -> Result<()> {
        // Usar un CATextLayer por hint o un CALayer por batch con glyph atlas.
        // Cada hint se pinta con posiciones absolutas y se reusa caché de atributos de texto.
    }
}
```

- **Windows**: overlay con layered window (`WS_EX_LAYERED | WS_EX_TRANSPARENT`) y Direct2D/DirectWrite para texto con cache de text layouts; alpha pre-multiplicado y `UpdateLayeredWindow` para batch.
- **Linux**: overlay por X11 (`override_redirect`, XShape para click-through) o Wayland layer-shell si está disponible; render con Cairo/Pango o Skia; en Wayland usar región de input vacía.
- **Rendimiento**: evitar WebView, precalcular glyphs, batch de draw calls, doble buffer para evitar flicker, y throttling a 60 fps máx (normalmente estático).

### 16.4 Keyboard Input Handler (nativo, sin WebView)

```rust
// src-tauri/src/services/hotkey_service.rs (modo navegación activo)

pub struct NavigationInput {
    buffer: String,
    last_keystroke: Instant,
    timeout_ms: u64,
}

impl NavigationInput {
    pub fn new() -> Self {
        Self { buffer: String::new(), last_keystroke: Instant::now(), timeout_ms: 1000 }
    }

    pub fn handle_key(&mut self, key: char) -> Option<String> {
        let now = Instant::now();
        if now.duration_since(self.last_keystroke).as_millis() > self.timeout_ms as u128 {
            self.buffer.clear();
        }
        self.last_keystroke = now;

        let valid = "asdfghjkl";
        if !valid.contains(key) {
            return None;
        }

        self.buffer.push(key);
        Some(self.buffer.clone())
    }
}

// macOS: CGEventTap en el overlay, nivel Session, captura keyDown y filtra modifiers.
// Windows: WH_KEYBOARD_LL hook filtrando vkCodes; evitar interferir con IME.
// Linux: evdev/raw XInput o key snooping vía compositor (Wayland limita; fallback a listener de atajos globales).
// El buffer se compara contra hints en Rust y dispara perform_click directamente sin pasar por JS.
```

### 16.5 Smart Element Filtering

```rust
// src-tauri/src/services/element_filter.rs

pub struct ElementFilter;

impl ElementFilter {
    /// Filtra elementos para mostrar solo los más relevantes
    pub fn filter_elements(elements: Vec<UIElement>) -> Vec<UIElement> {
        let mut filtered = Vec::new();
        
        // 1. Eliminar duplicados por posición
        let mut seen_positions = HashSet::new();
        
        // 2. Priorizar elementos visibles
        for element in elements {
            // Skip si está fuera de la pantalla
            if !Self::is_on_screen(&element) {
                continue;
            }
            
            // Skip si está oculto
            if Self::is_hidden(&element) {
                continue;
            }
            
            // Skip si ya hay un elemento en la misma posición
            let pos_key = (element.position.x as i32, element.position.y as i32);
            if seen_positions.contains(&pos_key) {
                continue;
            }
            seen_positions.insert(pos_key);
            
            // Skip si es demasiado pequeño para clickear
            if element.position.width < 10.0 || element.position.height < 10.0 {
                continue;
            }
            
            filtered.push(element);
        }
        
        // 3. Ordenar por relevancia (de arriba a abajo, izquierda a derecha)
        filtered.sort_by(|a, b| {
            a.position.y.partial_cmp(&b.position.y)
                .unwrap_or(std::cmp::Ordering::Equal)
                .then_with(|| {
                    a.position.x.partial_cmp(&b.position.x)
                        .unwrap_or(std::cmp::Ordering::Equal)
                })
        });
        
        filtered
    }
    
    fn is_on_screen(element: &UIElement) -> bool {
        // TODO: Obtener dimensiones de pantalla
        let screen_width = 3000.0;  // Placeholder
        let screen_height = 2000.0; // Placeholder
        
        element.position.x >= 0.0
            && element.position.y >= 0.0
            && element.position.x < screen_width
            && element.position.y < screen_height
    }
    
    fn is_hidden(element: &UIElement) -> bool {
        // Verificar atributo de visibilidad
        // TODO: Implementar check real via Accessibility API
        false
    }
}
```

### 16.6 Configuration Management

```rust
// src-tauri/src/models/config.rs

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub hotkeys: HotkeyConfig,
    pub appearance: AppearanceConfig,
    pub behavior: BehaviorConfig,
    pub blacklist: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HotkeyConfig {
    pub activate: String,           // "Cmd+J"
    pub activate_links_only: String, // "Cmd+K"
    pub cancel: String,             // "Escape"
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppearanceConfig {
    pub hint_background_color: String,
    pub hint_text_color: String,
    pub hint_font_size: u32,
    pub hint_border_width: u32,
    pub highlight_color: String,
    pub animation_duration: u32, // ms
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BehaviorConfig {
    pub auto_update_hints: bool,
    pub show_hint_count: bool,
    pub sound_feedback: bool,
    pub max_hints: usize,
    pub scan_depth: usize,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            hotkeys: HotkeyConfig {
                activate: "Cmd+J".to_string(),
                activate_links_only: "Cmd+K".to_string(),
                cancel: "Escape".to_string(),
            },
            appearance: AppearanceConfig {
                hint_background_color: "#fbbf24".to_string(),
                hint_text_color: "#000000".to_string(),
                hint_font_size: 14,
                hint_border_width: 2,
                highlight_color: "#3b82f6".to_string(),
                animation_duration: 150,
            },
            behavior: BehaviorConfig {
                auto_update_hints: true,
                show_hint_count: true,
                sound_feedback: false,
                max_hints: 500,
                scan_depth: 10,
            },
            blacklist: vec![],
        }
    }
}

impl AppConfig {
    pub fn load() -> Result<Self> {
        let config_path = Self::get_config_path()?;
        
        if config_path.exists() {
            let contents = std::fs::read_to_string(&config_path)?;
            Ok(serde_json::from_str(&contents)?)
        } else {
            Ok(Self::default())
        }
    }
    
    pub fn save(&self) -> Result<()> {
        let config_path = Self::get_config_path()?;
        let contents = serde_json::to_string_pretty(self)?;
        std::fs::write(&config_path, contents)?;
        Ok(())
    }
    
    fn get_config_path() -> Result<PathBuf> {
        let mut path = dirs::config_dir()
            .ok_or(AppError::ConfigError("No config dir".into()))?;
        path.push("KeyboardNav");
        std::fs::create_dir_all(&path)?;
        path.push("config.json");
        Ok(path)
    }
}
```

### 16.7 Pipeline de detección y asignación de hints (alta performance)

- **Escaneo incremental**: suscribirse a `AXObserver` (macOS), UIA `StructureChangedEvent` (Windows) y AT-SPI `Object:ChildrenChanged` (Linux) para evitar scans completos; al activarse, hacer un diff sobre el subárbol de la app enfocada.
- **BFS con límites adaptativos**: recorrer por anchura con `scan_depth` dinámico (más profundo solo para contenedores visibles y grandes); abortar ramas ocultas o fuera de viewport.
- **Hash estable por elemento**: `element_key = hash(role, title, frame, app_pid, path_en_arbol)` para reusar hints y evitar parpadeos; mantener un `HashMap<element_key, Hint>` y solo regenerar los nuevos.
- **Normalización de coordenadas**: convertir a coordenadas absolutas de pantalla con awareness multi-monitor y escalas (retina/HiDPI); descartar elementos con área < 10x10 px o fuera de screen bounds.
- **Filtrado corporativo**: blacklist configurable de apps/window titles; roles clicables whitelisted; score por visibilidad (alpha/hidden), tamaño y proximidad al centro de la ventana activa.
- **Asignación de labels**: ordenar por posición (top->bottom, left->right) para estabilidad; mapear con generador base-N del home row; mantener hints estables entre scans para muscle memory.

### 16.8 Estrategia de renderizado de hints (algoritmo)

- **Batch de texto con atlas**: pre-rasterizar el set de chars del home row en un atlas; cada hint se pinta como quad con UVs (Metal/Direct2D/Cairo) reduciendo draw calls.
- **Layout directo**: posiciones absolutas ya calculadas en Rust; sin DOM ni layout engine. Un buffer de comandos (`Vec<DrawCmd>`) se envía al renderer y se pinta de una sola pasada.
- **Double buffering**: dos layers/buffers para evitar flicker; se intercambian en `CATransaction` (macOS), `BeginDraw/EndDraw` (D2D), o surface swap (Cairo/Wayland).
- **Ciclo de vida corto**: overlay invisible hasta que hay hints; cuando está visible no repinta a más de 60 fps y solo si cambian los hints o se solicita highlight.
- **Accesos de entrada**: ventana click-through y focusless; teclado manejado en Rust (hotkey) y Svelte solo para configuración.

### 16.9 Optimización extrema (spatial, SIMD, pooling, pipeline)

```rust
// Spatial index con R-tree
use rstar::{RTree, AABB};

pub struct SpatialIndex {
    tree: RTree<IndexedElement>,
}

struct IndexedElement {
    bounds: AABB<[f64; 2]>,
    element: UIElement,
}

impl SpatialIndex {
    pub fn query_region(&self, rect: Rect) -> Vec<&UIElement> {
        self.tree
            .locate_in_envelope(&rect.to_aabb())
            .map(|e| &e.element)
            .collect()
    }
}

// SIMD para filtrar en pantalla
use std::simd::*;
pub fn filter_on_screen_simd(elements: &[UIElement], screen: Rect) -> Vec<UIElement> {
    // Procesar 4-8 elementos por iteración; caer a escalar para remanente.
    // Reducir comparaciones y branches en hot path.
    todo!()
}

// Memory pool para evitar allocs
pub struct ElementPool {
    pool: Vec<UIElement>,
    in_use: usize,
}
impl ElementPool {
    pub fn take(&mut self) -> &mut UIElement {
        if self.in_use == self.pool.len() {
            self.pool.push(UIElement::default());
        }
        let idx = self.in_use;
        self.in_use += 1;
        &mut self.pool[idx]
    }
    pub fn reset(&mut self) { self.in_use = 0; }
}

// Pipeline no bloqueante (scan -> hints -> render)
pub struct NonBlockingPipeline {
    scan_tx: mpsc::Sender<ScanRequest>,
    hint_tx: mpsc::Sender<Vec<UIElement>>,
    render_tx: mpsc::Sender<Vec<Hint>>,
}
// Cada etapa en thread dedicado, sin esperas entre stages.

// Plataforma
// macOS: Metal compute para posicionamiento/alpha y CALayer para compose.
// Windows: Direct2D + DirectComposition con layered window para batch updates.
// Linux: Cairo/Pango; fallback a Skia/Vulkan si Cairo no da throughput.
```

---

## 17. Performance Monitoring

```rust
// src-tauri/src/utils/performance.rs

use std::time::Instant;
use log::info;

pub struct PerformanceMonitor {
    start_times: HashMap<String, Instant>,
}

impl PerformanceMonitor {
    pub fn new() -> Self {
        Self {
            start_times: HashMap::new(),
        }
    }
    
    pub fn start(&mut self, label: &str) {
        self.start_times.insert(label.to_string(), Instant::now());
    }
    
    pub fn end(&mut self, label: &str) {
        if let Some(start) = self.start_times.remove(label) {
            let duration = start.elapsed();
            info!("{}: {:?}", label, duration);
            
            // Alertar si es muy lento
            if duration.as_millis() > 100 {
                log::warn!("{} took longer than expected: {:?}", label, duration);
            }
        }
    }
}

// Uso en código
let mut perf = PerformanceMonitor::new();
perf.start("scan_elements");
let elements = accessibility_service.scan_clickable_elements().await?;
perf.end("scan_elements");
```

### 17.1 Benchmarks y profiling

- **Criterion**: `benches/hint_generation.rs` para medir generación base-N y filtrado; targets: 100 hints < 5ms, 500 hints < 10ms.
- **puffin/optick** (feature `profiling`): marcar `scan_elements`, `filter_on_screen`, `render_hints`.
- **Objetivos actualizados**: scan < 50ms (incremental), hints < 5ms, render < 16ms (60fps), memoria < 30MB, latencia de click < 200ms.
- **Cargo profiles**: `lto = "fat"`, `codegen-units = 1`, `opt-level = 3`, `panic = "abort"`, `strip = true`.

---

## 18. Deployment & Distribution

### 18.1 macOS Code Signing

```bash
# Firmar la aplicación
codesign --deep --force --verify --verbose \
  --sign "Developer ID Application: Your Name (TEAM_ID)" \
  --options runtime \
  ./target/release/bundle/macos/KeyboardNav.app

# Notarizar con Apple
xcrun notarytool submit \
  ./target/release/bundle/macos/KeyboardNav.dmg \
  --apple-id "your@email.com" \
  --team-id "TEAM_ID" \
  --password "app-specific-password" \
  --wait

# Staple del ticket de notarización
xcrun stapler staple ./target/release/bundle/macos/KeyboardNav.dmg
```

### 18.2 Auto-Updates

```rust
// src-tauri/src/main.rs

use tauri::updater::UpdaterBuilder;

#[tauri::command]
async fn check_for_updates(app: AppHandle) -> Result<bool, String> {
    let update = app.updater()
        .check()
        .await
        .map_err(|e| e.to_string())?;
    
    Ok(update.is_some())
}

#[tauri::command]
async fn install_update(app: AppHandle) -> Result<(), String> {
    if let Some(update) = app.updater().check().await.map_err(|e| e.to_string())? {
        update.download_and_install().await.map_err(|e| e.to_string())?;
    }
    Ok(())
}
```

### 18.3 Analytics (Privacy-Focused)

```rust
// src-tauri/src/services/analytics.rs

pub struct Analytics {
    session_id: String,
    events: Vec<Event>,
}

#[derive(Serialize)]
struct Event {
    name: String,
    timestamp: u64,
    properties: HashMap<String, String>,
}

impl Analytics {
    pub fn track_activation(&mut self) {
        self.events.push(Event {
            name: "navigation_activated".to_string(),
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            properties: HashMap::new(),
        });
    }
    
    pub fn track_click(&mut self, hint_length: usize) {
        let mut props = HashMap::new();
        props.insert("hint_length".to_string(), hint_length.to_string());
        
        self.events.push(Event {
            name: "element_clicked".to_string(),
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            properties: props,
        });
    }
    
    // Solo enviar estadísticas agregadas, nunca datos personales
    pub async fn flush(&mut self) -> Result<()> {
        // Implementar envío a servidor de analytics
        Ok(())
    }
}
```

---

## 19. Documentación

### README.md Estructura
```markdown
# KeyboardNav

Navigate macOS with your keyboard like a pro.

## Features
- ⌨️ Keyboard-driven UI navigation
- 🎯 Intelligent hint generation
- ⚡ Lightning fast performance
- 🎨 Customizable appearance
- 🔒 Privacy-first (no data collection)

## Installation
[Installation instructions]

## Usage
[Usage guide with screenshots]

## Configuration
[Configuration options]

## Development
[Development setup]

## Contributing
[Contributing guidelines]

## License
MIT
```

---

## 20. Métricas de Éxito

### KPIs Técnicos
- Tiempo de scanning: < 100ms
- Tiempo de generación de hints: < 10ms
- Memoria usada: < 50MB
- CPU en idle: < 1%
- Tasa de falsos positivos: < 5%

### KPIs de Usuario
- Tiempo hasta primer click: < 500ms
- Precisión de clicks: > 95%
- Satisfacción del usuario: > 4.5/5
- Tasa de retención: > 70% a 30 días

---

Esta arquitectura proporciona una base sólida, escalable y profesional para construir una aplicación de navegación por teclado de nivel enterprise. La separación clara de responsabilidades, el manejo robusto de errores, y las consideraciones de performance la hacen production-ready.
