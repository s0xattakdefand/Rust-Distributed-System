#[derive(Debug, Clone)]
pub struct Shard {
    pub id: usize,
    pub data: Vec<String>,
}

impl Shard {
    pub fn new(id: usize) -> Self {
        Self {
            id,
            data: Vec::new(),
        }
    }

    pub fn store(&mut self, value: String) {
        self.data.push(value);
    }

    pub fn get_data(&self) -> Vec<String> {
        self.data.clone()
    }
}
