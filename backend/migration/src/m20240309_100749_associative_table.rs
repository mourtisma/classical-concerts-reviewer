use sea_orm_migration::prelude::*;

use crate::{m20240302_175801_relations::ExampleSeaOrmWithRelation, m20240303_190010_many_to_many::ExampleManyToMany};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .create_table(
                Table::create()
                    .table(ExampleSeaOrmWithRelationExampleManyToMany::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ExampleSeaOrmWithRelationExampleManyToMany::ExampleSeaOrmWithRelationId)
                            .uuid()
                            .not_null()
                    )
                    .col(
                        ColumnDef::new(ExampleSeaOrmWithRelationExampleManyToMany::ExampleManyToManyId)
                            .uuid()
                            .not_null()
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-example--with-relation_id")
                            .from(ExampleSeaOrmWithRelationExampleManyToMany::Table, ExampleSeaOrmWithRelationExampleManyToMany::ExampleSeaOrmWithRelationId)
                            .to(ExampleSeaOrmWithRelation::Table, ExampleSeaOrmWithRelation::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-example-many-to-many_id")
                            .from(ExampleSeaOrmWithRelationExampleManyToMany::Table, ExampleSeaOrmWithRelationExampleManyToMany::ExampleManyToManyId)
                            .to(ExampleManyToMany::Table, ExampleManyToMany::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(ExampleSeaOrmWithRelationExampleManyToMany::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum ExampleSeaOrmWithRelationExampleManyToMany {
    Table,
    ExampleSeaOrmWithRelationId,
    ExampleManyToManyId,
}
