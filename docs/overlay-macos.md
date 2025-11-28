# Overlay nativo (macOS) – Qué hace y cómo usarlo

## Resumen rápido
- Implementación en `src-tauri/src/platform/macos/overlay.rs`.
- Crea una `NSWindow` transparente, sin bordes, click-through y con capa raíz (`CALayer`).
- Cada hint se pinta como un `CATextLayer` con padding y esquinas redondeadas.
- Recalcula frame y escala (Retina) en cada `draw_hints` usando la pantalla principal.
- Limpia capas al ocultar (`hide`) y en `teardown` (`Drop`).

## API que te toca
- `WindowManager::show_overlay(&hints)` y `hide_overlay()` siguen siendo tu punto de entrada.
- `WindowManager` usa internamente el trait `OverlayRenderer` y el renderer macOS (`MacOverlay`).
- Si llamas a `show_overlay` desde Tauri, asegúrate de ejecutarlo en el hilo principal (usa `app.run_on_main_thread` si lo envías desde otro hilo) porque `MacOverlay` valida el hilo con `MainThreadMarker`.

## Coordenadas y escala
- Recibe coords de hints en el sistema “origen arriba/izquierda”; se convierten a AppKit (origen abajo) con `convert_y`.
- Se usa `backingScaleFactor` de la pantalla principal para `contentsScale`; así los textos no se ven borrosos en Retina.
- Ajusta el frame de ventana y capa raíz en cada `draw_hints` para reflejar cambios de resolución; por ahora toma solo la pantalla principal.

## Colores/tamaño
- Tamaño de fuente base `14.0` con padding extra para evitar cortes; fondo/colores aún mínimos (solo texto, sin `CGColor` custom).
- Esquinas redondeadas (`HINT_CORNER_RADIUS`) ya aplicadas en el `CATextLayer`.

## Limitaciones actuales
- Solo implementado para macOS; otros OS usan `NoopRenderer`.
- Multi-monitor: se toma `NSScreen::mainScreen()`; si mueves la ventana activa a otro monitor necesitarás detectar pantalla activa antes de pintar.
- Los comandos Tauri aún no forzan main-thread; si llamas desde un hilo de fondo fallará la guardia.

## Dónde tocar
- Ajustar colores/background: dentro de `create_hint_layer`.
- Soporte multi-monitor: antes de `screen_info` determina la pantalla de la ventana activa y usa ese frame.
- Integrar main-thread desde comandos: envolver `wm.show_overlay`/`hide_overlay` en `app.run_on_main_thread` o similar.

## Sintaxis/FFI que se usa
- Crates `objc2-*`:
  - `NSWindow::initWithContentRect_styleMask_backing_defer` (unsafe) para crear la ventana.
  - `CALayer` / `CATextLayer` para pintar texto.
  - `MainThreadMarker::new()` para garantizar hilo principal.
- Geometría via `objc2_core_foundation::{CGPoint, CGRect, CGSize}` (públicos con feature `CFCGTypes`).
- No hay `dispatch` ni `core-text`; toda la API es la expuesta por `objc2`/AppKit/QuartzCore.
