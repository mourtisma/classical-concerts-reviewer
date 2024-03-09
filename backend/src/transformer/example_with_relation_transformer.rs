use crate::{dto::example_with_relation_dto::ExampleWithRelationGetDto, model::prelude::{ExampleManyToManyModel, ExampleSeaOrmWithRelation, ExampleSeaOrmWithRelationModel}};

use super::example_many_to_many_transformer::ExampleManyToManyTransformer;

pub struct ExampleWithRelationTransformer {}

impl ExampleWithRelationTransformer {
    pub fn entity_to_get_dto(entity: (ExampleSeaOrmWithRelationModel, Vec<ExampleManyToManyModel>)) -> ExampleWithRelationGetDto {
        let (example_with_relation, examples_many_to_many) = entity;
        let examples_many_to_many_dtos = examples_many_to_many.iter().map(|e| ExampleManyToManyTransformer::entity_to_get_dto(e.clone())).collect();

        ExampleWithRelationGetDto {
            id: example_with_relation.id.to_string(),
            example_many_to_manys: examples_many_to_many_dtos,
            created_at: example_with_relation.created_at.to_string(),
            updated_at: example_with_relation.updated_at.to_string()
        }
    }
}