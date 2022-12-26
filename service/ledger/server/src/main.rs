use ledger_core::{
    config::{get_config, get_log_level_filter},
    context::Context,
};
use ledger_server::server::Server;

use env_logger;
use log::info;
use std::{env, sync::Arc};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    setup();

    let config = get_config();

    info!("Starting...");

    let ctx = Arc::new(Context::init(config).await?);

    let server: Server = Server::new(ctx).await?;
    server.run().await
}

fn setup() {
    let log_level = env::var("LOG_LEVEL").unwrap_or_else(|_| "warn".to_string());
    env_logger::builder()
        .filter_level(get_log_level_filter(log_level))
        .is_test(true)
        .init();
}
