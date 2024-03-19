use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(ExampleSeaOrm::Table)
                    .modify_column(ColumnDef::new(ExampleSeaOrm::CreatedAt).date_time().default(Expr::current_timestamp()).not_null())
                    .modify_column(ColumnDef::new(ExampleSeaOrm::UpdatedAt).date_time().default(Expr::current_timestamp()).not_null())
                    .to_owned()
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                .table(ExampleSeaOrm::Table)
                .modify_column(ColumnDef::new(ExampleSeaOrm::CreatedAt).date_time().default(Expr::current_date()).not_null())
                .modify_column(ColumnDef::new(ExampleSeaOrm::UpdatedAt).date_time().default(Expr::current_date()).not_null())
                .to_owned()
            )
            .await
    }
}

#[derive(DeriveIden)]
enum ExampleSeaOrm {
    Table,
    Id,
    Name,
    CreatedAt,
    UpdatedAt
}
