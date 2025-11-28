#![allow(dead_code)]

use crate::models::UIElement;

/// Región rectangular para queries espaciales
#[derive(Debug, Clone, Copy)]
pub struct Region {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

impl Region {
    pub fn new(x: f64, y: f64, width: f64, height: f64) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    /// Región que cubre toda la pantalla (viewport completo)
    pub fn full_screen() -> Self {
        Self::new(0.0, 0.0, f64::MAX, f64::MAX)
    }
}

/// Índice espacial para queries eficientes de elementos por región
/// TODO: Implementar con rstar::RTree en Fase 3
#[allow(dead_code)]
pub struct SpatialIndex {
    elements: Vec<UIElement>,
}

impl SpatialIndex {
    pub fn new() -> Self {
        Self {
            elements: Vec::new(),
        }
    }

    /// Inserta un elemento en el índice
    #[allow(dead_code)]
    pub fn insert(&mut self, element: UIElement) {
        // TODO: Implementar inserción en R-tree
        self.elements.push(element);
    }

    /// Inserta múltiples elementos en el índice
    #[allow(dead_code)]
    pub fn insert_batch(&mut self, elements: Vec<UIElement>) {
        // TODO: Implementar inserción batch en R-tree
        self.elements.extend(elements);
    }

    /// Consulta elementos dentro de una región
    #[allow(dead_code)]
    pub fn query_region(&self, _region: &Region) -> Vec<&UIElement> {
        // TODO: Implementar query espacial con R-tree
        // Por ahora devuelve todos los elementos
        self.elements.iter().collect()
    }

    /// Limpia el índice
    #[allow(dead_code)]
    pub fn clear(&mut self) {
        self.elements.clear();
    }

    /// Número de elementos en el índice
    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.elements.len()
    }

    /// Verifica si el índice está vacío
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }
}

impl Default for SpatialIndex {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spatial_index_new() {
        let index = SpatialIndex::new();
        assert!(index.is_empty());
    }

    #[test]
    fn test_region_full_screen() {
        let region = Region::full_screen();
        assert_eq!(region.x, 0.0);
        assert_eq!(region.y, 0.0);
    }
}
