use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Account::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Account::Id)
                            .binary_len(16)
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Account::OwnerId).binary_len(16).not_null())
                    .col(ColumnDef::new(Account::Currency).string().not_null())
                    .col(
                        ColumnDef::new(Account::Balance)
                            .decimal_len(14, 4)
                            .not_null(),
                    )
                    .col(ColumnDef::new(Account::UpdatedAt).date_time_len(4).null())
                    .col(
                        ColumnDef::new(Account::CreatedAt)
                            .date_time_len(4)
                            .not_null(),
                    )
                    .index(
                        Index::create()
                            .name("owner_id_idx")
                            .table(Account::Table)
                            .col(Account::OwnerId),
                    )
                    .index(
                        Index::create()
                            .name("updated_at_idx")
                            .table(Account::Table)
                            .col(Account::UpdatedAt),
                    )
                    .index(
                        Index::create()
                            .name("created_at_idx")
                            .table(Account::Table)
                            .col(Account::CreatedAt),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Account::Table).to_owned().to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum Account {
    Table,
    Id,
    OwnerId,
    Currency,
    Balance,
    UpdatedAt,
    CreatedAt,
}
