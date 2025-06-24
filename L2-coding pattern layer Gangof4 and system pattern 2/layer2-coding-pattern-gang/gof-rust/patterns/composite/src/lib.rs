//! File-system Composite.

pub trait Component {
    fn size(&self) -> u64;
}

pub struct File {
    bytes: u64,
}
impl File {
    pub fn new(bytes: u64) -> Self {
        Self { bytes }
    }
}
impl Component for File {
    fn size(&self) -> u64 {
        self.bytes
    }
}

pub struct Directory {
    children: Vec<Box<dyn Component>>,
}
impl Directory {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
        }
    }
    pub fn add(&mut self, c: Box<dyn Component>) {
        self.children.push(c);
    }
}
impl Component for Directory {
    fn size(&self) -> u64 {
        self.children.iter().map(|c| c.size()).sum()
    }
}
impl Default for Directory {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn total() {
        let mut dir = Directory::new();
        dir.add(Box::new(File::new(10)));
        dir.add(Box::new(File::new(20)));
        assert_eq!(dir.size(), 30);
    }
}
