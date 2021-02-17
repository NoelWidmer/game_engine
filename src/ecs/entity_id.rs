#[derive(Debug, Copy, Clone, PartialEq, Hash)]
pub struct EntityId {
    id: u64
}

impl EntityId {
    pub fn new(id: u64) -> Self {
        Self { id }
    }
}

impl Eq for EntityId { }

impl std::fmt::Display for EntityId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.id)
    }
}