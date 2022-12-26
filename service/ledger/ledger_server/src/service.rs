use tonic::{Request, Response, Status};

use crate::ledger::ledger_service_server::LedgerService;
use crate::ledger::{NewAccountRequest, NewAccountResponse};

#[derive(Default)]
pub struct LedgerServiceImpl {}

#[tonic::async_trait]
impl LedgerService for LedgerServiceImpl {
    async fn new_account(
        &self,
        request: Request<NewAccountRequest>,
    ) -> Result<Response<NewAccountResponse>, Status> {
        let response = NewAccountResponse {
            value: format!("Hello {}!", request.into_inner().value),
        };

        Ok(Response::new(response))
    }
}
