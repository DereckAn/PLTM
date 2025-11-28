---

# Arquitectura General – PLTM (Keyboard Navigation App)

Aplicación multi-OS tipo Homerow, ultra optimizada, con arquitectura empresarial y foco obsesivo en performance.

---

## 1. Objetivos

1. **Producto**

   - Navegación por teclado para macOS / Windows / Linux.
   - Overlay de hints **100% nativo** (sin WebView) y configurable.
   - UI de configuración moderna (Svelte).

2. **Arquitectura Enterprise**

   - Capas bien separadas: Presentation → Application → Business Logic → Platform.
   - Código mantenible, testeable y fácil de extender.

3. **Performance extrema (literalmente optimizar todo)**

   - Overlay renderizado por GPU.
   - Escaneo **incremental** del árbol de accesibilidad.
   - R-tree, SIMD, memory pooling, pipeline no bloqueante.
   - Targets de latencia tipo “motor de juego”.

---

## 2. Stack Tecnológico

### Core

- **Backend/Core**: Rust + Tauri 2.x
- **Frontend**: Svelte 5 + TailwindCSS + TypeScript (solo settings / permisos)
- **Empaquetado**: Tauri multi-OS
- **Package Manager**: Bun

### Rendering (Overlay sin WebView)

- **macOS**: CoreGraphics + CoreAnimation + Metal (compute shaders / GPU batching)
- **Windows**: Direct2D + DirectWrite + DirectComposition (layered window)
- **Linux**: Cairo + Pango + Skia (backend Vulkan cuando sea posible)

### Performance & Calidad

- **Benchmarking**: Criterion
- **Profiling**: puffin, Tracy
- **Testing**:

  - Frontend: Vitest / Playwright
  - Backend: Rust tests (unit + integration)

- **CI/CD**: GitHub Actions (build, tests, benchmarks, releases, auto-updates)

---

## 3. Arquitectura por Capas

```text
┌─────────────────────────────────────────────────────────────┐
│                     PRESENTATION LAYER                       │
│  ┌────────────────────────────────────────────────────────┐ │
│  │   Svelte Settings UI (settings, permisos, theming)     │ │
│  │   Native Overlay Renderer (CG/Metal/D2D/Cairo/Skia)    │ │
│  └────────────────────────────────────────────────────────┘ │
└─────────────────────────────────────────────────────────────┘
                              ▼
┌─────────────────────────────────────────────────────────────┐
│                    APPLICATION LAYER                         │
│  ┌────────────────────────────────────────────────────────┐ │
│  │       Tauri Command Interface + Event System           │ │
│  │   • IPC Bridge                                         │ │
│  │   • State Management                                   │ │
│  │   • Config & Persistence                               │ │
│  └────────────────────────────────────────────────────────┘ │
└─────────────────────────────────────────────────────────────┘
                              ▼
┌─────────────────────────────────────────────────────────────┐
│                      BUSINESS LOGIC LAYER                    │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────────┐  │
│  │ Accessibility│  │   Hotkey     │  │   Click          │  │
│  │   Service    │  │   Service    │  │   Service        │  │
│  └──────────────┘  └──────────────┘  └──────────────────┘  │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────────┐  │
│  │   Hint       │  │   Window     │  │   Config         │  │
│  │ Generator    │  │   Manager    │  │   Manager        │  │
│  └──────────────┘  └──────────────┘  └──────────────────┘  │
└─────────────────────────────────────────────────────────────┘
                              ▼
┌─────────────────────────────────────────────────────────────┐
│               PLATFORM LAYER (macOS / Win / Linux)          │
│  ┌────────────────────────────────────────────────────────┐ │
│  │       Native FFI Bindings per OS                       │ │
│  │  • Accesibilidad (AX / UIA / AT-SPI2)                  │ │
│  │  • Eventos globales (keyboard/mouse)                   │ │
│  │  • Overlay Renderer (Metal / D2D / Cairo+Skia)         │ │
│  └────────────────────────────────────────────────────────┘ │
└─────────────────────────────────────────────────────────────┘
```

---

## 4. Estructura de Proyecto

```text
keyboard-nav-app/
├── src/                                   # Frontend (Svelte)
│   ├── lib/
│   │   ├── components/
│   │   │   ├── settings/
│   │   │   │   ├── SettingsPanel.svelte
│   │   │   │   ├── HotkeyConfig.svelte
│   │   │   │   └── AppearanceConfig.svelte
│   │   │   └── permissions/
│   │   │       └── PermissionGuard.svelte
│   │   ├── stores/
│   │   │   ├── app-state.ts
│   │   │   └── settings.ts
│   │   ├── services/
│   │   │   ├── tauri-commands.ts
│   │   │   └── event-listener.ts
│   │   └── utils/
│   │       └── types.ts
│   ├── routes/
│   │   └── +page.svelte
│   └── app.css
│
├── src-tauri/                             # Backend (Rust)
│   ├── src/
│   │   ├── main.rs
│   │   ├── lib.rs
│   │   ├── state/
│   │   │   └── app_state.rs
│   │   ├── models/
│   │   │   ├── element.rs
│   │   │   ├── hint.rs
│   │   │   └── config.rs
│   │   ├── services/                      # Business Logic
│   │   │   ├── accessibility_service.rs
│   │   │   ├── hotkey_service.rs
│   │   │   ├── click_service.rs
│   │   │   ├── hint_generator.rs
│   │   │   ├── window_manager.rs
│   │   │   ├── element_filter.rs
│   │   │   ├── spatial_index.rs
│   │   │   └── pipeline.rs
│   │   ├── platform/                      # Platform-specific
│   │   │   ├── macos/
│   │   │   │   ├── accessibility.rs
│   │   │   │   ├── overlay.rs
│   │   │   │   └── events.rs
│   │   │   ├── windows/
│   │   │   │   ├── accessibility.rs
│   │   │   │   ├── overlay.rs
│   │   │   │   └── events.rs
│   │   │   └── linux/
│   │   │       ├── accessibility.rs
│   │   │       ├── overlay.rs
│   │   │       └── events.rs
│   │   ├── commands/                      # Tauri Commands
│   │   │   ├── accessibility.rs
│   │   │   ├── hotkey.rs
│   │   │   └── window.rs
│   │   ├── utils/
│   │   │   ├── logger.rs
│   │   │   └── performance.rs
│   │   └── error/
│   │       └── app_error.rs
│   ├── Cargo.toml
│   └── tauri.conf.json
│
├── tests/
│   ├── unit/
│   ├── integration/
│   └── e2e/
├── .github/workflows/
│   ├── ci.yml
│   └── release.yml
├── package.json
├── bun.lockb
├── tailwind.config.js
├── svelte.config.js
└── README.md
```

---

## 5. Sistema de Modos (Vim-style)

### Modos

1. **Idle** – OS tiene control completo.
2. **Navigation** – captura de teclas (j/k/gg/G/f/F…).
3. **Hint Left** – hints amarillos (click izquierdo).
4. **Hint Right** – hints azules (click derecho / middle / modificadores).
5. **Search** – filtros/áreas específicas.

### Flujo

```text
Cmd+J → Navigation
  f   → generar/mostrar hints izquierdos
  F   → hints derechos
  j/k → scroll vertical
  gg  → scroll top
  G   → scroll bottom
  Esc → volver a Idle
```

La lógica de modos vive en Business Logic (HotkeyService + NavigationInput) y no depende de la UI.

---

## 6. Servicios Core (Business Logic Layer)

### 6.1 AccessibilityService

Responsable de:

- Verificar y solicitar permisos de accesibilidad.
- Escanear el árbol de accesibilidad (AX / UIA / AT-SPI2).
- Generar una lista de `UIElement` con posición absoluta en pantalla.
- Integrarse con el **pipeline incremental** (observers).

Puntos clave:

- Uso de **BFS con profundidad adaptativa** (`scan_depth`).
- Integración con observers por OS:

  - macOS: `AXObserver`.
  - Windows: UIA `StructureChangedEvent`.
  - Linux: AT-SPI2 `Object:ChildrenChanged`.

- API pensada para:

  - `full_scan()` para casos extremos.
  - `incremental_scan(diff)` para uso normal.

---

### 6.2 HotkeyService + NavigationInput

Responsable de:

- Registrar hotkeys globales (Cmd+J, etc.).
- Cambiar de modo (Idle / Navigation / Search).
- Capturar secuencia de caracteres para hints:

  - Buffer con timeout (ej. 1s).
  - Solo acepta charset de hints (ej. `asdfghjkl`).

---

### 6.3 HintGenerator

- Genera labels tipo Vimium:

  - Base-N sobre charset del home row (ej. `a,s,d,f,j,k,l,h`).
  - Objetivo: para 100–500 elementos, longitud 2–3 chars máx.

- Mantiene estabilidad de labels usando claves por elemento:

  - `hash(role, title, frame, app_pid, tree_path)`.

---

### 6.4 ClickService

- Simula clicks nativos:

  - Normal, con modificadores (Cmd, Shift), middle, etc.

- 100% en la capa de Plataforma:

  - macOS: CGEvent.
  - Windows: SendInput.
  - Linux: XTest / input del compositor.

---

### 6.5 WindowManager + OverlayRenderer

- Interfaz `OverlayRenderer`:

  - `init()`, `draw_hints(&[Hint])`, `show()`, `hide()`.

- Implementaciones por OS:

  - **macOS**: NSWindow transparente + CALayer + Metal.
  - **Windows**: layered window (`WS_EX_LAYERED | WS_EX_TRANSPARENT`) + Direct2D + DirectComposition.
  - **Linux**: X11 override_redirect o layer-shell en Wayland + Cairo/Skia.

Responsable de:

- Mantener el overlay siempre **click-through**.
- Respetar multi-monitor y HiDPI.
- Implementar **double buffering** para evitar flicker.
- Usar **glyph atlas** y **batch draw** para hints.

---

## 7. Capa de Plataforma (Platform Layer)

Cada OS tiene un módulo que expone funciones “limpias” a Business Logic:

- `accessibility::*`

  - `has_accessibility_permissions()`
  - `request_permissions()`
  - `get_active_window()`
  - `traverse_accessibility_tree(...)`

- `events::*`

  - Registro de hooks de teclado.
  - Post de eventos de ratón.

- `overlay::*`

  - Implementación de `OverlayRenderer`.

Todos los detalles feos (C, punteros, tipos CoreFoundation, COM, etc.) quedan encerrados aquí.

---

## 8. Performance – Principios Globales

Aquí se combinan tus dos planes: enterprise + nivel bestia.

### 8.1 Filosofía

1. **Más vale evitar trabajo que hacer trabajo rápido.**
2. **Todo hot path está pensado como “motor de juego” (frame < 16 ms).**
3. **Nada bloquea el input del usuario.**
4. **Primero escanear lo mínimo posible, luego pintar lo mínimo posible.**
5. **Medir todo** (profiling continuo).

---

## 9. Pipeline No Bloqueante (Scan → Process → Render)

Tres etapas principales:

```text
[Scan Thread]    → obtiene UIElements (incremental, con observers)
[Process Thread] → filtra, indexa, genera hints (R-tree, SIMD, pooling)
[Render Thread]  → dibuja overlay con GPU y swap de buffers
```

Comunicación por **canales lock-free** (`mpsc`) para minimizar contención.

### 9.1 Scan Stage

- Usa `AccessibilityService`.
- Apoyado por observers + BFS con profundidad adaptativa.
- Output: lista “bruta” de `UIElement` (+ diff incremental).
- Targets:

  - Escenario normal: < **50 ms** en incremental.
  - Full scan >500 elementos: raramente, pero apuntar a < 100 ms.

### 9.2 Process Stage

Responsable de:

1. **Hash-based Element Tracking**

   - Calcular `ElementHash` estable.
   - Reusar hints antiguos cuando sea posible (evitar parpadeos).

2. **SIMD Filtering**

   - Filtrado masivo de:

     - Elementos fuera de pantalla.
     - Demasiado pequeños.
     - Duplicados en misma posición.

   - Uso de `std::simd` para checar bounds de varios elementos a la vez.

3. **Spatial Index (R-tree)**

   - `rstar::RTree<IndexedElement>`.
   - Queries rápidas por región visible (viewport) y para features futuras (por ejemplo, selección contextual).

4. **Memory Pooling**

   - Pools para `UIElement` y `Hint` que se resetean en cada activación.
   - Minimizar allocs en hot path de activación (Cmd+J → hints).

5. **Hint Generation**

   - Ordenar elementos por posición (top→bottom, left→right).
   - Generar labels base-N.
   - Mantener estabilidad usando el hash estable.

### 9.3 Render Stage

- Usa `OverlayRenderer` en cada OS.
- Optimizado para:

  - **Glyph atlas**: pre-rasterizar el charset (a,s,d,f,j,k,l,h + números).
  - **Batch draw**: un buffer de draw commands por frame.
  - **Double buffering**: dos layers/buffers que se swapean de forma atómica.

- Solo re-renderiza cuando:

  - Se generan nuevos hints.
  - Cambia el highlight (cuando el usuario escribe la secuencia).

---

## 10. Optimizaciones Críticas (Resumen)

1. **R-Tree Spatial Indexing**

   - `O(log n)` para queries espaciales.
   - Útil para viewport culling y lógica futura.

2. **SIMD Vectorization**

   - 4–8 elementos procesados en paralelo.
   - Uso en:

     - Filtrar por bounds de pantalla.
     - Cálculo de distancias / orden por proximidad si lo necesitas.

3. **Memory Pooling**

   - Pools pre-alocados para estructuras hot (`UIElement`, `Hint`).
   - Evitas `malloc`/`free` repetidas y fragmentación.

4. **Hash-based Element Tracking**

   - Hints estables → UX pro.
   - Menos trabajo de regeneración.

5. **Incremental Scanning con Observers**

   - Dejas de hacer full scans constantes.
   - Solo re-escaneas ramas del árbol que cambian.

6. **BFS con Límites Adaptativos**

   - Más profundidad solo donde tiene sentido (contenedores visibles grandes).
   - Abortas ramas ocultas/irrelevantes temprano.

7. **Glyph Atlas Caching**

   - Un solo batch de draw para todos los hints.
   - Menos draw calls = más FPS y menos CPU.

8. **Double Buffering**

   - Sin flicker ni tearing.
   - Menos trabajo innecesario al no mezclar lectura/escritura.

9. **Non-Blocking Pipeline**

   - Threads dedicados por etapa.
   - El UI thread / input thread jamás se bloquea.

10. **GPU Acceleration (Metal / D2D / Skia/Vulkan)**

    - La CPU se dedica al escaneo y filtrado.
    - El dibujo pesado lo hace la GPU.

11. **Lazy Viewport Culling**

    - Solo se pintan hints en el viewport visible.
    - Si en el futuro hay zoom/splits/multi-window, el R-tree lo soporta.

12. **Zero-Copy FFI**

    - Evitar serializar elementos innecesariamente entre Rust y APIs nativas.
    - Idealmente, trabajar con referencias/punteros en Platform Layer y solo copiar lo mínimo para Business Logic.

---

## 11. Targets de Performance

| Métrica               | Target       |
| --------------------- | ------------ |
| Scan incremental      | < **50 ms**  |
| Generación de hints   | < **5 ms**   |
| Render overlay        | < **16 ms**  |
| Latencia hotkey→hints | < **200 ms** |
| Memoria total         | < **30 MB**  |
| CPU en idle           | < **1 %**    |

Estas métricas se validan en:

- Escenario 100 elementos.
- Escenario 500 elementos (stress test).
- Diferentes resoluciones (1080p, 1440p, 4K, multi-monitor).

---

## 12. Estrategia de Testing & Profiling

### 12.1 Tests

- **Unit**:

  - `HintGenerator` (unicidad, longitud, estabilidad).
  - `ElementFilter` (on-screen / tamaño / duplicados).
  - `SpatialIndex` (queries correctas).

- **Integration**:

  - Workflow: permisos → scan → hints → click.

- **E2E**:

  - Activar hotkey y validar que aparecen hints en apps reales (Playwright / harness propio).

### 12.2 Profiling

- `puffin` / `Tracy` para:

  - `scan_elements`
  - `process_elements`
  - `generate_hints`
  - `render_hints`

- `Criterion` para:

  - Benchmarks de generación de hints.
  - Filtrado y spatial indexing.

- CI:

  - Comparar benchmarks contra baseline.
  - Alertar si hay regresión > 10%.

---

## 13. Roadmap (Fases combinadas)

### Fase 1 – Foundation (Arquitectura + MVP lento pero limpio)

- [x] Setup Tauri + Svelte + Bun.
- [x] Capa Platform macOS básica: permisos AX, PID, ventana, roles/título/rect; overlay y clicks con stubs nativos mínimos.
- [x] Services y models: AppError/Result/logging listos; AccessibilityService parcial; otros servicios esqueleto.
- [~] Hotkey básico, scan simple, hints sin R-tree ni SIMD: hotkey registra combo fijo; scan hace BFS y mapea a UIElement; hint gen simple.
- [ ] Overlay nativo que dibuja hints (aún stub).

### Fase 2 – Core Pipeline

- [ ] Separar threads: Scan, Process, Render.
- [~] Implementar `ElementFilter`: filtrado por rol/tamaño está; falta desduplicado/spatial/visibilidad.
- [~] Añadir `HintGenerator` estable: generador base-N simple, integración incompleta.
- [ ] Integrar `WindowManager` con `OverlayRenderer`: sigue stub, sin render nativo.

### Fase 3 – Performance v1 (Rápido)

- [ ] Integrar **R-tree** (`rstar`).
- [ ] Implementar **BFS adaptativo**.
- [ ] Añadir **hash estable** por elemento.
- [ ] Agregar **PerformanceMonitor** básico para tiempos por etapa.

### Fase 4 – Performance v2 (Muy rápido)

- [ ] Añadir **SIMD** en filtrado.
- [ ] Implementar **memory pooling** en hot paths.
- [ ] Pulir pipeline no bloqueante (canales y backpressure).

### Fase 5 – Rendering Beast Mode

- [ ] Implementar **glyph atlas** serio.
- [ ] Optimizar render en Metal / D2D / Skia.
- [ ] Activar **double buffering** fino y throttling (no más de 60fps).

### Fase 6 – Pulido Final

- [ ] Tests unitarios/integración/E2E sólidos.
- [ ] Benchmarks en CI y dashboards.
- [ ] Code signing + auto-updates.
- [ ] Documentación y web de producto.

---

Si quieres, en el siguiente mensaje puedo:

- Generarte este doc ya con **front-matter para Docusaurus** o similar, o
- Bajar cada sección a **checklists técnicas** para que las pegues como issues en GitHub (por feature/performance).
