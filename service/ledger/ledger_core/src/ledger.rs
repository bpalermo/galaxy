use crate::entity::{account, account::Entity as Account, txn};
use crate::error::LedgerError;

use migration_lib::{Migrator, MigratorTrait};
use sea_orm::{prelude::Decimal, *};
use sea_query::{query::LockType, Expr};
use std::sync::Arc;
use util_lib::util;
use uuid::Uuid;

#[derive(FromQueryResult)]
struct AccountBalance {
    balance: Decimal,
}

#[derive(Debug)]
pub struct Ledger {
    db: Arc<DatabaseConnection>,
}

impl Default for Ledger {
    fn default() -> Self {
        Self {
            db: Arc::new(DatabaseConnection::Disconnected),
        }
    }
}

impl Ledger {
    pub async fn new(
        db: Arc<DatabaseConnection>,
        migrate: Option<bool>,
    ) -> Result<Self, LedgerError> {
        if migrate.unwrap_or(true) {
            Migrator::up(&*db, None).await?;
        }
        Ok(Self { db })
    }

    pub async fn new_account(
        &self,
        owner_id: Uuid,
        currency: String,
    ) -> Result<account::Model, LedgerError> {
        let acc = account::ActiveModel {
            owner_id: Set(owner_id),
            currency: Set(currency),
            ..Default::default()
        }
        .insert(&*self.db)
        .await?;

        Ok(acc)
    }

    pub async fn get_account_by_id(&self, id: Uuid) -> Result<Option<account::Model>, LedgerError> {
        let account: Option<account::Model> = Account::find_by_id(id).one(&*self.db).await?;
        Ok(account)
    }

    pub async fn new_deposit(
        &self,
        account_id: Uuid,
        amount: Decimal,
    ) -> Result<txn::Model, LedgerError> {
        let db_txn = (&*self.db).begin().await?;
        let txn: txn::Model =
            Ledger::new_txn(&db_txn, account_id, amount, txn::TxnType::Deposit).await?;
        Ledger::add_balance(&db_txn, account_id, amount).await?;
        db_txn.commit().await?;
        Ok(txn)
    }

    pub async fn new_withdraw(
        &self,
        account_id: Uuid,
        amount: Decimal,
    ) -> Result<txn::Model, LedgerError> {
        let db_txn = (&*self.db).begin().await?;
        let txn: txn::Model =
            Ledger::new_txn(&db_txn, account_id, amount, txn::TxnType::Withdraw).await?;
        Ledger::sub_balance(&db_txn, account_id, amount).await?;
        db_txn.commit().await?;
        Ok(txn)
    }

    async fn new_txn(
        db_txn: &DatabaseTransaction,
        account_id: Uuid,
        amount: Decimal,
        txn_type: txn::TxnType,
    ) -> Result<txn::Model, LedgerError> {
        let t = txn::ActiveModel {
            account_id: Set(account_id),
            amount: Set(amount),
            txn_type: Set(txn_type),
            ..Default::default()
        }
        .insert(db_txn)
        .await?;

        Ok(t)
    }

    async fn add_balance(
        db_txn: &DatabaseTransaction,
        account_id: Uuid,
        amount: Decimal,
    ) -> Result<(), LedgerError> {
        let update_result: UpdateResult = Account::update_many()
            .col_expr(
                account::Column::Balance,
                Expr::col(account::Column::Balance).add(amount),
            )
            .col_expr(
                account::Column::UpdatedAt,
                Expr::value(Some(util::Util::now())),
            )
            .filter(Expr::col(account::Column::Id).eq(account_id))
            .exec(db_txn)
            .await?;

        if update_result.rows_affected != 1 {
            return Err(LedgerError::AddBalanceError);
        }

        Ok(())
    }

    async fn sub_balance(
        db_txn: &DatabaseTransaction,
        account_id: Uuid,
        amount: Decimal,
    ) -> Result<(), LedgerError> {
        let balance = Ledger::get_balance_by_account_id::<Account>(
            db_txn,
            account_id,
            Some(LockType::Update),
        )
        .await?;

        if balance < amount {
            return Err(LedgerError::InsufficientBalanceError);
        }

        let result: UpdateResult = Account::update_many()
            .col_expr(
                account::Column::Balance,
                Expr::col(account::Column::Balance).sub(amount),
            )
            .filter(
                Condition::all()
                    .add(account::Column::Id.eq(account_id))
                    .add(account::Column::Balance.gte(amount)),
            )
            .exec(db_txn)
            .await?;

        if result.rows_affected != 1 {
            return Err(LedgerError::InsufficientBalanceError);
        }

        Ok(())
    }

    pub async fn add_balance_by_owner(
        &self,
        id: Uuid,
        owner_id: Uuid,
        amount: Decimal,
    ) -> Result<bool, LedgerError> {
        let update_result: UpdateResult = Account::update_many()
            .col_expr(
                account::Column::Balance,
                Expr::col(account::Column::Balance).add(amount),
            )
            .col_expr(
                account::Column::UpdatedAt,
                Expr::value(Some(util::Util::now())),
            )
            .filter(
                Expr::col(account::Column::Id)
                    .eq(id)
                    .and(Expr::col(account::Column::OwnerId).eq(owner_id)),
            )
            .exec(&*self.db)
            .await?;

        Ok(update_result.rows_affected > 0)
    }

    pub async fn sub_balance_by_owner(
        &self,
        id: Uuid,
        owner_id: Uuid,
        amount: Decimal,
    ) -> Result<(), LedgerError> {
        let txn = (&*self.db).begin().await?;

        let balance = Ledger::get_balance_by_id_and_owner::<Account>(
            &txn,
            id,
            owner_id,
            Some(LockType::Update),
        )
        .await?;

        if balance < amount {
            return Err(LedgerError::InsufficientBalanceError);
        }

        let result: UpdateResult = Account::update_many()
            .col_expr(
                account::Column::Balance,
                Expr::col(account::Column::Balance).sub(amount),
            )
            .filter(
                Condition::all()
                    .add(account::Column::Id.eq(id))
                    .add(account::Column::OwnerId.eq(owner_id))
                    .add(account::Column::Balance.gte(amount)),
            )
            .exec(&txn)
            .await?;

        if result.rows_affected != 1 {
            return Err(LedgerError::InsufficientBalanceError);
        }

        txn.commit().await?;

        Ok(())
    }

    async fn get_balance_by_id_and_owner<E: EntityTrait>(
        txn: &DatabaseTransaction,
        id: Uuid,
        owner_id: Uuid,
        lock_type: Option<LockType>,
    ) -> Result<Decimal, LedgerError> {
        let mut select: Select<Account> = Account::find()
            .select_only()
            .column(account::Column::Balance)
            .filter(
                Condition::all()
                    .add(account::Column::Id.eq(id))
                    .add(account::Column::OwnerId.eq(owner_id)),
            );

        if let Some(lt) = lock_type {
            select = select.lock(lt)
        }

        let account_balance: Option<AccountBalance> =
            select.into_model::<AccountBalance>().one(txn).await?;

        if let Some(account) = account_balance {
            Ok(account.balance)
        } else {
            Err(LedgerError::AccountNotFoundError)
        }
    }

    async fn get_balance_by_account_id<E: EntityTrait>(
        db_txn: &DatabaseTransaction,
        account_id: Uuid,
        lock_type: Option<LockType>,
    ) -> Result<Decimal, LedgerError> {
        let mut select: Select<Account> = Account::find()
            .select_only()
            .column(account::Column::Balance)
            .filter(Condition::all().add(account::Column::Id.eq(account_id)));

        if let Some(lt) = lock_type {
            select = select.lock(lt)
        }

        let account_balance: Option<AccountBalance> =
            select.into_model::<AccountBalance>().one(db_txn).await?;

        if let Some(account) = account_balance {
            Ok(account.balance)
        } else {
            Err(LedgerError::AccountNotFoundError)
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::entity::account::Entity as Account;
    use crate::error::LedgerError;
    use sea_orm::prelude::Decimal;
    use std::sync::Arc;
    use util_lib::util;
    use uuid::{Uuid, Version};

    #[tokio::test]
    async fn test_new_account() -> Result<(), LedgerError> {
        let id = Uuid::new_v4();
        let owner_id = Uuid::new_v4();
        let currency = "BRL";
        let balance = Decimal::new(0, 0);

        let db = MockDatabase::new(DatabaseBackend::MySql)
            .append_query_results(vec![vec![account::Model {
                id,
                owner_id,
                currency: currency.to_owned(),
                balance,
                updated_at: None,
                created_at: util::Util::now(),
            }]])
            .append_exec_results(vec![MockExecResult {
                last_insert_id: 1,
                rows_affected: 1,
            }])
            .into_connection();

        let l = Ledger { db: Arc::new(db) };

        let acc = l.new_account(owner_id, currency.to_owned()).await?;
        assert_ne!(Uuid::nil(), acc.id);
        assert_eq!(Some(Version::Random), acc.id.get_version());
        assert_eq!(owner_id, acc.owner_id);
        assert_eq!(currency.to_owned(), acc.currency);
        assert_eq!(balance, acc.balance);

        Ok(())
    }

    #[tokio::test]
    async fn test_add_balance_by_owner_success() -> Result<(), LedgerError> {
        let id = Uuid::new_v4();
        let owner_id = Uuid::new_v4();

        let db = MockDatabase::new(DatabaseBackend::MySql)
            .append_exec_results(vec![MockExecResult {
                last_insert_id: 1,
                rows_affected: 1,
            }])
            .into_connection();
        let l = Ledger { db: Arc::new(db) };

        let result = l
            .add_balance_by_owner(id, owner_id, Decimal::new(10, 0))
            .await?;
        assert!(result);

        Ok(())
    }

    #[tokio::test]
    async fn test_add_balance_by_owner_error() -> Result<(), LedgerError> {
        let id = Uuid::new_v4();
        let owner_id = Uuid::new_v4();

        let db = MockDatabase::new(DatabaseBackend::MySql)
            .append_exec_results(vec![])
            .into_connection();
        let l = Ledger { db: Arc::new(db) };

        assert_eq!(
            l.add_balance_by_owner(id, owner_id, Decimal::new(10, 0))
                .await,
            Err(LedgerError::DatabaseError)
        );

        Ok(())
    }

    #[tokio::test]
    async fn test_get_balance_by_id_and_owner_success() -> Result<(), LedgerError> {
        let id = Uuid::new_v4();
        let owner_id = Uuid::new_v4();
        let currency = "BRL";
        let balance = Decimal::new(10, 0);

        let db = MockDatabase::new(DatabaseBackend::MySql)
            .append_query_results(vec![vec![account::Model {
                id,
                owner_id,
                currency: currency.to_owned(),
                balance: Decimal::new(10, 0),
                updated_at: None,
                created_at: util::Util::now(),
            }]])
            .into_connection();
        let txn = db.begin().await?;

        let actual: Decimal =
            Ledger::get_balance_by_id_and_owner::<Account>(&txn, id, owner_id, None).await?;
        assert_eq!(balance, actual);

        Ok(())
    }

    #[tokio::test]
    async fn test_get_balance_by_id_and_owner_error() -> Result<(), LedgerError> {
        let id = Uuid::new_v4();
        let owner_id = Uuid::new_v4();

        let db = MockDatabase::new(DatabaseBackend::MySql)
            .append_query_results::<account::Model>(vec![vec![]])
            .into_connection();
        let txn = db.begin().await?;

        assert_eq!(
            Ledger::get_balance_by_id_and_owner::<Account>(&txn, id, owner_id, None).await,
            Err(LedgerError::AccountNotFoundError)
        );

        Ok(())
    }
}
