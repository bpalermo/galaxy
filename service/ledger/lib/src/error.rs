use std::{error, fmt};

#[derive(Eq, Debug, PartialEq)]
pub enum LedgerError {
    AccountNotFoundError,
    AddBalanceError,
    DatabaseError,
    InsufficientBalanceError,
    TransactionError,
}

impl error::Error for LedgerError {}

impl fmt::Display for LedgerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LedgerError::AccountNotFoundError => write!(f, "Account was not found"),
            LedgerError::AddBalanceError => write!(f, "Add balance error"),
            LedgerError::DatabaseError => write!(f, "Database error"),
            LedgerError::InsufficientBalanceError => write!(f, "Insufficient account balance"),
            LedgerError::TransactionError => write!(f, "Transaction error"),
        }
    }
}

impl From<sea_orm::DbErr> for LedgerError {
    fn from(_: sea_orm::DbErr) -> Self {
        LedgerError::DatabaseError
    }
}

impl<E: std::error::Error> From<sea_orm::TransactionError<E>> for LedgerError {
    fn from(e: sea_orm::TransactionError<E>) -> Self {
        match e {
            sea_orm::TransactionError::Transaction(_) => LedgerError::TransactionError,
            sea_orm::TransactionError::Connection(_) => LedgerError::DatabaseError,
        }
    }
}
