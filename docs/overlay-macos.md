# Overlay nativo (macOS) – Qué hace y cómo usarlo

## Resumen rápido
- Implementación en `src-tauri/src/platform/macos/overlay.rs`.
- Crea una `NSWindow` transparente, sin bordes, click-through y con capa raíz (`CALayer`).
- Cada hint se pinta como un `CATextLayer` con padding y esquinas redondeadas.
- Recalcula frame y escala (Retina) en cada `draw_hints` usando la pantalla principal.
- Limpia capas al ocultar (`hide`) y en `teardown` (`Drop`).

## API que te toca
- `WindowManager::show_overlay(&hints)` y `hide_overlay()` siguen siendo el entrypoint.
- `WindowManager` usa el trait `OverlayRenderer` y el renderer macOS (`MacOverlay`).
- Los comandos Tauri (`show_hints`, `activate_navigation`) ya despachan el render al hilo principal con `app.run_on_main_thread`, no necesitas hacer nada extra en frontend.

## Coordenadas y escala
- Recibe coords de hints en el sistema “origen arriba/izquierda”; se convierten a AppKit (origen abajo) con `convert_y`.
- Se usa `backingScaleFactor` de la pantalla principal para `contentsScale`; así los textos no se ven borrosos en Retina.
- Ajusta el frame de ventana y capa raíz en cada `draw_hints` para reflejar cambios de resolución; por ahora toma solo la pantalla principal.

## Colores/tamaño
- Tamaño de fuente base `14.0` con padding extra para evitar cortes; fondo/colores aún mínimos (solo texto, sin `CGColor` custom).
- Esquinas redondeadas (`HINT_CORNER_RADIUS`) ya aplicadas en el `CATextLayer`.

## Limitaciones actuales
- Solo implementado para macOS; otros OS usan `NoopRenderer`.
- Multi-monitor: se toma `NSScreen::mainScreen()`; falta detectar la pantalla activa antes de pintar.
- Menús/toolbar/systray: aún no se capturan hints para menú superior ni barra de estado.

## Dónde tocar
- Ajustar colores/background: dentro de `create_hint_layer`.
- Soporte multi-monitor: antes de `screen_info` determina la pantalla de la ventana activa y usa ese frame.
- Integrar main-thread desde comandos: envolver `wm.show_overlay`/`hide_overlay` en `app.run_on_main_thread` o similar.

## Sintaxis/FFI que se usa
- Crates `objc2-*`:
  - `NSWindow::initWithContentRect_styleMask_backing_defer` (unsafe) para crear la ventana.
  - `CALayer` / `CATextLayer` para pintar texto.
  - `MainThreadMarker::new()` para garantizar hilo principal (las llamadas vienen del comando ya en main thread).
- Geometría via `objc2_core_foundation::{CGPoint, CGRect, CGSize}` (públicos con feature `CFCGTypes`).
