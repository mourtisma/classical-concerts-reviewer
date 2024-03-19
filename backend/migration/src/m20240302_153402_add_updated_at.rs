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
                    .add_column(ColumnDef::new(ExampleSeaOrm::UpdatedAt).date_time().default(Expr::current_date()).not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(Table::alter().table(ExampleSeaOrm::Table).drop_column(ExampleSeaOrm::UpdatedAt).to_owned())
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
