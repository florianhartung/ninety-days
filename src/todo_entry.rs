/// A dataclass for a single to-do list entry
#[derive(Debug, Clone)]
pub struct TodoEntry {
    pub id: u32,
    pub name: String,
    pub is_done: bool,
}

impl TodoEntry {
    /// Create a new undone to-do list entry
    pub fn new(id: u32, name: String) -> Self {
        Self {
            id,
            name,
            is_done: false,
        }
    }
}
