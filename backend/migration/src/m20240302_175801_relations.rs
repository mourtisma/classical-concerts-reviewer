use sea_orm_migration::prelude::*;
use super::m20220101_000001_create_table::ExampleSeaOrm;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(ExampleSeaOrmWithRelation::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ExampleSeaOrmWithRelation::Id)
                            .uuid()
                            .extra("DEFAULT gen_random_uuid()")
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(ExampleSeaOrmWithRelation::ExampleId).uuid().not_null())
                    .col(ColumnDef::new(ExampleSeaOrmWithRelation::CreatedAt).date_time().default(Expr::current_timestamp()).not_null())
                    .col(ColumnDef::new(ExampleSeaOrmWithRelation::UpdatedAt).date_time().default(Expr::current_timestamp()).not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-example-relation_id")
                            .from(ExampleSeaOrmWithRelation::Table, ExampleSeaOrmWithRelation::ExampleId)
                            .to(ExampleSeaOrm::Table, ExampleSeaOrm::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(ExampleSeaOrmWithRelation::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum ExampleSeaOrmWithRelation {
    Table,
    Id,
    ExampleId,
    CreatedAt,
    UpdatedAt
}
