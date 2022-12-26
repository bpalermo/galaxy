use crate::error::LedgerError;
use crate::{config::Config, ledger::Ledger};

use sea_orm::{Database, DatabaseConnection};
use std::sync::Arc;

/// Dependencies needed
#[derive(Debug)]
pub struct Context {
    /// The app config
    pub config: &'static Config,
    /// The database connections
    pub db: Arc<DatabaseConnection>,
    /// The ledger
    pub ledger: Arc<Ledger>,
}

/// Initialize dependencies
impl Context {
    pub async fn init(config: &'static Config) -> Result<Self, LedgerError> {
        let db = Arc::new(Database::connect(&config.database.url).await?);
        let ledger = Arc::new(Ledger::new(db.clone(), None).await?);

        Ok(Self { config, db, ledger })
    }
}
