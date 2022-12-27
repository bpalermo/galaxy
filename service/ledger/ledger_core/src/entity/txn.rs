use sea_orm::{entity::prelude::*, ActiveValue};
use time::PrimitiveDateTime;
use util_lib::util;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "txn")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub account_id: Uuid,
    pub txn_type: TxnType,
    pub amount: Decimal,
    pub updated_at: Option<PrimitiveDateTime>,
    pub created_at: PrimitiveDateTime,
}

#[derive(Clone, Eq, EnumIter, Debug, DeriveActiveEnum, PartialEq)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "txn_type")]
pub enum TxnType {
    #[sea_orm(string_value = "DEPOSIT")]
    Deposit,
    #[sea_orm(string_value = "WITHDRAW")]
    Withdraw,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::account::Entity",
        from = "Column::AccountId",
        to = "super::account::Column::Id"
    )]
    Account,
}

impl Related<super::account::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Account.def()
    }
}

impl ActiveModelBehavior for ActiveModel {
    /// Create a new ActiveModel with default values. Also used by `Default::default()`.
    fn new() -> Self {
        Self {
            id: ActiveValue::Set(Uuid::new_v4()),
            created_at: ActiveValue::Set(util::Util::now()),
            ..ActiveModelTrait::default()
        }
    }

    /// Will be triggered before insert / update
    fn before_save(mut self, insert: bool) -> Result<Self, DbErr> {
        if !insert {
            self.updated_at = ActiveValue::Set(Some(util::Util::now()));
        }

        Ok(self)
    }
}

#[cfg(test)]
mod tests {
    use sea_orm::{entity::*, DatabaseBackend, MockDatabase, MockExecResult};

    use super::*;

    #[tokio::test]
    async fn test_insert_transaction() -> Result<(), DbErr> {
        let id = Uuid::new_v4();
        let account_id = Uuid::new_v4();
        let amount = Decimal::new(203402, 4);
        let now = TimeDateTimeWithTimeZone::now_utc();
        let created_at = TimeDateTime::new(now.date(), now.time());

        let db = MockDatabase::new(DatabaseBackend::MySql)
            .append_query_results(vec![vec![Model {
                id,
                account_id,
                txn_type: TxnType::Deposit,
                amount,
                updated_at: None,
                created_at,
            }]])
            .append_exec_results(vec![MockExecResult {
                last_insert_id: 1,
                rows_affected: 1,
            }])
            .into_connection();

        let account = ActiveModel {
            id: Set(id),
            account_id: Set(account_id),
            txn_type: Set(TxnType::Deposit),
            amount: Set(amount),
            updated_at: NotSet,
            created_at: Set(created_at),
        };

        // Insert the ActiveModel into MockDatabase
        assert_eq!(
            account.clone().insert(&db).await?,
            Model {
                id,
                account_id,
                txn_type: TxnType::Deposit,
                amount,
                updated_at: None,
                created_at,
            }
        );

        Ok(())
    }
}
