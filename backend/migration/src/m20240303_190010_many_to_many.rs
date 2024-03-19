use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .create_table(
                Table::create()
                    .table(ExampleManyToMany::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ExampleManyToMany::Id)
                        .uuid()
                        .extra("DEFAULT gen_random_uuid()")
                        .not_null()
                        .primary_key(),
                    )
                    .col(ColumnDef::new(ExampleManyToMany::Name).string().not_null())
                    .col(ColumnDef::new(ExampleManyToMany::CreatedAt).date_time().default(Expr::current_timestamp()).not_null())
                    .col(ColumnDef::new(ExampleManyToMany::UpdatedAt).date_time().default(Expr::current_timestamp()).not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .drop_table(Table::drop().table(ExampleManyToMany::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum ExampleManyToMany {
    Table,
    Id,
    Name,
    CreatedAt,
    UpdatedAt
}
