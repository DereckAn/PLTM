pub struct HintService {
    // Aquí podrías agregar campos necesarios para el servicio de hints
    sequence: HintSequence,
}

impl HintGenerator {
    pub fn generate_hints(&self, count: usize) -> Vec<String> {
        // Algoritmo para generar hints eficientes
        // Similar a vimium: usa caracteres del homerow 
    }

    // Estrategia de generacion: home row priority
    fn get_hint_chars(&self) -> Vec<char> {
        vec!['a', 's', 'd', 'f', 'j', 'k', 'l', 'h', 'g',';']
    }
}