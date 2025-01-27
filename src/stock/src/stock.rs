use std::fmt::Display;

pub struct Data {
    id: usize,
    name: String,
    stock: usize,
}

impl Data {
    pub fn new(id: usize, name: String, stock: usize) -> Self {
        Self { id, name, stock }
    }

    pub fn id(&self) -> usize {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn stock(&self) -> usize {
        self.stock
    }
}

impl PartialEq for Data {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Data {}

impl PartialOrd for Data {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.id.cmp(&other.id))
    }
}

impl Ord for Data {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.id.cmp(&other.id)
    }
}

impl Display for Data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.id)
    }
}
