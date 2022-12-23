use sea_orm::{Database, DatabaseConnection, DbErr};
use std::sync::Arc;

/// Dependencies needed
#[derive(Debug)]
pub struct Context {
    /// The database connections
    pub db: Arc<DatabaseConnection>,
}

/// Initialize dependencies
impl Context {
    pub async fn init(db_url: &str) -> Result<Self, DbErr> {
        let db = Arc::new(Database::connect(db_url).await?);

        Ok(Self { db })
    }
}
