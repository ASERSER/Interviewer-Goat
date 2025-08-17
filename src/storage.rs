use anyhow::Result;

/// Lightweight wrapper around persistent storage.
pub struct Database;

impl Database {
    /// Create a new database at the provided path.
    pub fn new(_path: &str) -> Result<Self> {
        // In a full implementation, this would initialize a connection using
        // `sqlx` and run any pending migrations.
        Ok(Self)
    }
}
