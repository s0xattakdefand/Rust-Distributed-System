//! Flyweight glyph interning.

use std::collections::HashMap;
use std::sync::Mutex;

#[derive(Clone)]
pub struct Glyph(char);
impl Glyph {
    pub fn draw(&self) -> char {
        self.0
    }
}

pub struct GlyphFactory {
    pool: Mutex<HashMap<char, Glyph>>,
}
impl GlyphFactory {
    pub fn new() -> Self {
        Self {
            pool: Mutex::new(HashMap::new()),
        }
    }
    pub fn get(&self, c: char) -> Glyph {
        let mut map = self.pool.lock().unwrap();
        map.entry(c).or_insert_with(|| Glyph(c)).clone()
    }
}
impl Default for GlyphFactory {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn shared() {
        let f = GlyphFactory::default();
        let a1 = f.get('a');
        let a2 = f.get('a');
        assert_eq!(a1.draw(), a2.draw());
    }
}
