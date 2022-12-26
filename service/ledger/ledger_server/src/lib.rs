pub mod ledger {
    tonic::include_proto!("galaxy.service.ledger.v1");
}

pub mod server;
pub mod service;
