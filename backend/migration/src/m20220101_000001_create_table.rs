use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(ExampleSeaOrm::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ExampleSeaOrm::Id)
                            .uuid()
                            .extra("DEFAULT gen_random_uuid()")
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(ExampleSeaOrm::Name).not_null().custom(Alias::new("citext")))
                    .col(ColumnDef::new(ExampleSeaOrm::CreatedAt).date_time().default(Expr::current_date()).not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(ExampleSeaOrm::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum ExampleSeaOrm {
    Table,
    Id,
    Name,
    CreatedAt,
}
