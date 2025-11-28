use crate::models::{Hint, UIElement};

/// Charset para generar hints (home row)
const HINT_CHARS: &str = "asdfghjkl";

/// Generador de hints tipo Vimium
pub struct HintGenerator {
    charset: Vec<char>,
}

impl HintGenerator {
    pub fn new() -> Self {
        tracing::debug!("Initializing HintGenerator");
        Self {
            charset: HINT_CHARS.chars().collect(),
        }
    }

    /// Crea un generador con charset personalizado
    #[allow(dead_code)]
    pub fn with_charset(charset: &str) -> Self {
        Self {
            charset: charset.chars().collect(),
        }
    }

    /// Genera hints para una lista de elementos
    pub fn generate(&self, elements: &[UIElement]) -> Vec<Hint> {
        tracing::debug!("Generating hints for {} elements", elements.len());

        let labels = self.generate_labels(elements.len());

        elements
            .iter()
            .zip(labels.into_iter())
            .map(|(element, label)| {
                // Calcular centro del elemento
                let center_x = element.x + element.width / 2.0;
                let center_y = element.y + element.height / 2.0;

                Hint::new(label, center_x, center_y, element.id.clone())
            })
            .collect()
    }

    /// Genera labels base-N para N elementos
    fn generate_labels(&self, count: usize) -> Vec<String> {
        if count == 0 {
            return Vec::new();
        }

        let base = self.charset.len();
        let mut labels = Vec::with_capacity(count);

        // Calcular cuántos dígitos necesitamos
        let digits = self.calculate_digits(count, base);

        for i in 0..count {
            labels.push(self.index_to_label(i, digits));
        }

        labels
    }

    /// Calcula cuántos dígitos se necesitan para representar N elementos
    fn calculate_digits(&self, count: usize, base: usize) -> usize {
        if count <= base {
            return 1;
        }

        let mut digits = 1;
        let mut capacity = base;

        while capacity < count {
            digits += 1;
            capacity *= base;
        }

        digits
    }

    /// Convierte un índice a un label base-N
    fn index_to_label(&self, mut index: usize, digits: usize) -> String {
        let base = self.charset.len();
        let mut result = Vec::with_capacity(digits);

        for _ in 0..digits {
            result.push(self.charset[index % base]);
            index /= base;
        }

        result.into_iter().rev().collect()
    }
}

impl Default for HintGenerator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_element(id: &str, x: f64, y: f64) -> UIElement {
        UIElement::new(
            id.to_string(),
            "AXButton".to_string(),
            None,
            x,
            y,
            100.0,
            30.0,
        )
    }

    #[test]
    fn test_generate_single_char_labels() {
        let gen = HintGenerator::new();
        let labels = gen.generate_labels(5);

        assert_eq!(labels.len(), 5);
        assert_eq!(labels[0], "a");
        assert_eq!(labels[1], "s");
        assert_eq!(labels[2], "d");
    }

    #[test]
    fn test_generate_double_char_labels() {
        let gen = HintGenerator::new();
        // Con 9 chars, necesitamos 2 dígitos para >9 elementos
        let labels = gen.generate_labels(15);

        assert_eq!(labels.len(), 15);
        assert_eq!(labels[0], "aa");
        assert_eq!(labels[9], "sa"); // Después de aa-al viene sa
    }

    #[test]
    fn test_generate_hints() {
        let gen = HintGenerator::new();
        let elements = vec![
            make_element("btn1", 10.0, 20.0),
            make_element("btn2", 100.0, 200.0),
        ];

        let hints = gen.generate(&elements);

        assert_eq!(hints.len(), 2);
        assert_eq!(hints[0].label, "a");
        assert_eq!(hints[1].label, "s");
        // Centro de btn1: (10 + 100/2, 20 + 30/2) = (60, 35)
        assert_eq!(hints[0].x, 60.0);
        assert_eq!(hints[0].y, 35.0);
    }

    #[test]
    fn test_empty_elements() {
        let gen = HintGenerator::new();
        let hints = gen.generate(&[]);
        assert!(hints.is_empty());
    }
}
