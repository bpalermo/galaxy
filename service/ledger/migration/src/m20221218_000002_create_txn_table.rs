use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Txn::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Txn::Id)
                            .binary_len(16)
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Txn::AccountId).binary_len(16).not_null())
                    .col(ColumnDef::new(Txn::TxnType).string().not_null())
                    .col(ColumnDef::new(Txn::Amount).decimal_len(14, 4).not_null())
                    .col(ColumnDef::new(Txn::UpdatedAt).date_time_len(4).null())
                    .col(ColumnDef::new(Txn::CreatedAt).date_time_len(4).not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_account_id")
                            .from(Txn::Table, Txn::AccountId)
                            .to(Account::Table, Account::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Restrict),
                    )
                    .index(
                        Index::create()
                            .name("account_id_idx")
                            .table(Txn::Table)
                            .col(Txn::AccountId),
                    )
                    .index(
                        Index::create()
                            .name("updated_at_idx")
                            .table(Txn::Table)
                            .col(Txn::UpdatedAt),
                    )
                    .index(
                        Index::create()
                            .name("created_at_idx")
                            .table(Txn::Table)
                            .col(Txn::CreatedAt),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Txn::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum Txn {
    Table,
    Id,
    AccountId,
    TxnType,
    Amount,
    UpdatedAt,
    CreatedAt,
}

#[derive(Iden)]
enum Account {
    Table,
    Id,
}
