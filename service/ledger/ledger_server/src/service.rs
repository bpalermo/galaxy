use account_v1_rust::galaxy::types::account::v1::Account;
use ledger_core::{entity::account, ledger::Ledger};
use ledger_v1_rust::galaxy::service::ledger::v1::ledger_service_server::LedgerService;
use ledger_v1_rust::galaxy::service::ledger::v1::{
    MyNewAccountRequest, MyNewAccountResponse, NewAccountRequest, NewAccountResponse,
};
use pbjson_types::Timestamp;
use std::sync::Arc;
use tonic::{Request, Response, Status};
use util_lib::auth::get_owner_id_from_subject_header;
use util_lib::proto::decimal_to_google_money;
use util_lib::uuid::parse_uuid_from_string;
use uuid::Uuid;

#[derive(Default)]
pub struct LedgerServer {
    ledger: Arc<Ledger>,
    subject_header: String,
}

impl LedgerServer {
    pub fn new(ledger: Arc<Ledger>, subject_header: String) -> Self {
        return Self {
            ledger,
            subject_header,
        };
    }
}

#[tonic::async_trait]
impl LedgerService for LedgerServer {
    async fn new_account(
        &self,
        req: Request<NewAccountRequest>,
    ) -> Result<Response<NewAccountResponse>, Status> {
        let request: NewAccountRequest = req.into_inner();
        let owner_id: Uuid = parse_uuid_from_string(request.owner_id)
            .ok_or(Status::invalid_argument("invalid owner ID"))?;

        let acc: account::Model = (&*self.ledger)
            .new_account(owner_id, request.currency.to_owned())
            .await
            .map_err(|_| Status::internal("internal error"))?;

        let mut updated_at = None;
        if let Some(value) = acc.updated_at {
            updated_at = Some(Timestamp {
                seconds: value.timestamp(),
                nanos: value.timestamp_subsec_nanos() as i32,
            })
        }

        let created_at = Some(Timestamp {
            seconds: acc.created_at.timestamp(),
            nanos: acc.created_at.timestamp_subsec_nanos() as i32,
        });

        let response = NewAccountResponse {
            account: Some(Account {
                id: acc.id.to_string(),
                owner_id: acc.owner_id.to_string(),
                balance: decimal_to_google_money(acc.currency, acc.balance),
                updated_at,
                created_at,
            }),
        };

        Ok(Response::new(response))
    }

    async fn my_new_account(
        &self,
        req: Request<MyNewAccountRequest>,
    ) -> Result<Response<MyNewAccountResponse>, Status> {
        let owner_id: Uuid = get_owner_id_from_subject_header(&req, &self.subject_header)
            .ok_or(Status::invalid_argument("invalid owner ID"))?;
        let request: MyNewAccountRequest = req.into_inner();

        let acc: account::Model = (&*self.ledger)
            .new_account(owner_id, request.currency.to_owned())
            .await
            .map_err(|_| Status::internal("internal error"))?;

        let response = MyNewAccountResponse {
            id: acc.id.to_string(),
            owner_id: acc.owner_id.to_string(),
            balance: decimal_to_google_money(acc.currency, acc.balance),
        };

        Ok(Response::new(response))
    }
}
