use sea_orm::{entity::prelude::*, ActiveValue};
use util_rust::util;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "account")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub owner_id: Uuid,
    pub currency: String,
    pub balance: Decimal,
    pub updated_at: Option<TimeDateTime>,
    pub created_at: TimeDateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::txn::Entity")]
    Txn,
}

impl Related<super::txn::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Txn.def()
    }
}

impl ActiveModelBehavior for ActiveModel {
    /// Create a new ActiveModel with default values. Also used by `Default::default()`.
    fn new() -> Self {
        Self {
            id: ActiveValue::Set(Uuid::new_v4()),
            balance: ActiveValue::Set(Decimal::new(0, 0)),
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
    async fn test_insert_account() -> Result<(), DbErr> {
        let id = Uuid::new_v4();
        let owner_id = Uuid::new_v4();
        let balance = Decimal::new(202, 2);
        let now = TimeDateTimeWithTimeZone::now_utc();
        let created_at = TimeDateTime::new(now.date(), now.time());

        let db = MockDatabase::new(DatabaseBackend::MySql)
            .append_query_results(vec![vec![Model {
                id,
                owner_id,
                currency: "BRL".to_owned(),
                balance,
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
            owner_id: Set(owner_id),
            currency: Set("BRL".to_owned()),
            balance: Set(balance),
            updated_at: NotSet,
            created_at: Set(created_at),
        };

        // Insert the ActiveModel into MockDatabase
        assert_eq!(
            account.clone().insert(&db).await?,
            Model {
                id,
                owner_id,
                currency: "BRL".to_owned(),
                balance,
                updated_at: None,
                created_at,
            }
        );

        Ok(())
    }
}
