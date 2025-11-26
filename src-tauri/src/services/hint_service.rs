use crate::Result;

pub struct HintGenerator {
    charset: Vec<char>,
}

impl HintGenerator {
    pub fn new() -> Self {
        Self {
            charset: vec!['a', 's', 'd', 'f', 'j', 'k', 'l', 'h', 'g', ';'],
        }
    }

    pub fn generate_hints(&self, count: usize) -> Result<Vec<String>> {
        let base = self.charset.len();
        let mut hints = Vec::with_capacity(count);
        for i in 0..count {
            hints.push(self.number_to_hint(i, base));
        }
        Ok(hints)
    }

    fn number_to_hint(&self, mut num: usize, base: usize) -> String {
        let mut hint = String::new();
        loop {
            let digit = num % base;
            hint.insert(0, self.charset[digit]);
            num /= base;
            if num == 0 {
                break;
            }
        }
        hint
    }
}
