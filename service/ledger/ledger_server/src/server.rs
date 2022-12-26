use crate::{ledger::ledger_service_server::LedgerServiceServer, service::LedgerServiceImpl};

use ledger_core::context::Context;
use ledger_core::error::LedgerError;
use log::info;
use std::sync::Arc;
use tonic::transport::Server as TonicServer;

#[derive(Debug)]
pub struct Server {
    ctx: Arc<Context>,
}

impl Server {
    pub async fn new(ctx: Arc<Context>) -> Result<Self, LedgerError> {
        Ok(Self { ctx })
    }

    pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        let port = self.ctx.config.port;
        let addr = &format!("0.0.0.0:{}", port).parse().unwrap();

        let (_, health_service) = tonic_health::server::health_reporter();

        info!("Server listening on {}", addr);

        TonicServer::builder()
            .add_service(health_service)
            .add_service(LedgerServiceServer::new(LedgerServiceImpl::default()))
            .serve(*addr)
            .await?;

        Ok(())
    }
}
