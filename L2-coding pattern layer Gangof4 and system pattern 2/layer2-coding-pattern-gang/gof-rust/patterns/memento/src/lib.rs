//! Text-editor Memento.

#[derive(Clone)]
pub struct Memento {
    text: String,
}

pub struct Editor {
    text: String,
}
impl Editor {
    pub fn new() -> Self {
        Self {
            text: String::new(),
        }
    }
    pub fn type_char(&mut self, ch: char) {
        self.text.push(ch);
    }
    pub fn save(&self) -> Memento {
        Memento {
            text: self.text.clone(),
        }
    }
    pub fn restore(&mut self, m: Memento) {
        self.text = m.text;
    }
    pub fn text(&self) -> &str {
        &self.text
    }
}
impl Default for Editor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn undo() {
        let mut ed = Editor::default();
        ed.type_char('a');
        let m = ed.save();
        ed.type_char('b');
        ed.restore(m);
        assert_eq!(ed.text(), "a");
    }
}
