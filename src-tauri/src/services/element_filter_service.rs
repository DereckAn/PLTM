use crate::models::UIElement;
use std::collections::HashSet;

/// Tamaño minimo para considerar un elemento visible
const MIN_SIZE: f64 = 5.0;

// Precision para redondear posiciones (deduplicacion)
const POSITION_PRECISION: i32 = 5;

// Filtra elementos UI aplicando reglas de visibilidad y deduplicacion
pub struct ElementFilter;

impl ElementFilter {
    pub fn new() -> Self {
        Self
    }

    // Filtra elementos aplicando todos los filtros
    pub fn filter(&self, elements: Vec<UIElement>) -> Vec<UIElement> {
        tracing::debug!(
            "Starting element filtering. Initial count: {}",
            elements.len()
        );

        let filtered = self.filter_by_size(elements);
        let filtered = self.deduplicate_by_position(filtered);

        tracing::debug!(
            "Finished element filtering. Final count: {}",
            filtered.len()
        );
        filtered
    }

    // Filtra elementos demasiado pequeños
    fn filter_by_size(&self, elements: Vec<UIElement>) -> Vec<UIElement> {
        elements
            .into_iter()
            .filter(|el| el.width > MIN_SIZE && el.height > MIN_SIZE)
            .collect()
    }

    // Elimina elementos duplicados en la misma posicion (redondeada)
    fn deduplicate_by_position(&self, elements: Vec<UIElement>) -> Vec<UIElement> {
        let mut seen: HashSet<(i32, i32)> = HashSet::new();
        let mut result = Vec::with_capacity(elements.len());

        for el in elements {
            let key = (
                (el.x as i32) / POSITION_PRECISION * POSITION_PRECISION,
                (el.y as i32) / POSITION_PRECISION * POSITION_PRECISION,
            );

            if seen.insert(key) {
                result.push(el);
            }
        }

        result
    }
}

impl Default for ElementFilter {
    fn default() -> Self {
        Self::new()
    }
}

// Funcion conveniente para filtrar elementos
pub fn filter_elements(elements: Vec<UIElement>) -> Vec<UIElement> {
    ElementFilter::new().filter(elements)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_element(id: &str, x: f64, y: f64, w: f64, h: f64) -> UIElement {
        UIElement::new(id.to_string(), "AXButton".to_string(), None, x, y, w, h)
    }

    #[test]
    fn test_filter_small_elements() {
        let elements = vec![
            make_element("a", 0.0, 0.0, 10.0, 10.0),
            make_element("b", 0.0, 0.0, 0.5, 0.5), // Too small
            make_element("c", 0.0, 0.0, 1.0, 0.5), // Height too small
        ];

        let filtered = filter_elements(elements);
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].id, "a");
    }

    #[test]
    fn test_deduplicate_by_position() {
        let elements = vec![
            make_element("a", 10.0, 10.0, 50.0, 20.0),
            make_element("b", 11.0, 11.0, 50.0, 20.0), // Same rounded position
            make_element("c", 100.0, 100.0, 50.0, 20.0),
        ];

        let filtered = filter_elements(elements);
        assert_eq!(filtered.len(), 2);
    }
}
