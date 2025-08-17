/// Minimal in-memory vector store placeholder used for RAG queries.
#[derive(Debug, Clone)]
pub struct VectorItem {
    pub content: String,
    pub score: f32,
}

pub struct VectorStore;

impl VectorStore {
    pub fn new() -> Self {
        Self
    }

    /// Perform a vector similarity search. Returns an empty result for now.
    pub fn query(&self, _query: &str) -> Vec<VectorItem> {
        Vec::new()
    }
}
