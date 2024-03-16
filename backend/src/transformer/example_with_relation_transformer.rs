use chrono::Utc;
use sea_orm::{ActiveValue::NotSet, Set};
use uuid::Uuid;

use crate::{dto::{example_many_to_many_dto, example_with_relation_dto::{ExampleWithRelationCreateDto, ExampleWithRelationGetDto, ExampleWithRelationUpdateDto}}, model::prelude::{ExampleManyToManyActiveModel, ExampleManyToManyModel, ExampleSeaOrmWithRelation, ExampleSeaOrmWithRelationModel, ExampleWithRelationActiveModel}};

use super::example_many_to_many_transformer::ExampleManyToManyTransformer;

pub struct ExampleWithRelationTransformer {}

impl<'a> ExampleWithRelationTransformer {
    pub fn entity_to_get_dto(entity: (ExampleSeaOrmWithRelationModel, Vec<ExampleManyToManyModel>)) -> ExampleWithRelationGetDto {
        let (example_with_relation, examples_many_to_many) = entity;
        let examples_many_to_many_dtos = examples_many_to_many.iter().map(|e| ExampleManyToManyTransformer::entity_to_get_dto(e.clone())).collect();

        ExampleWithRelationGetDto {
            id: example_with_relation.id.to_string(),
            example_id: example_with_relation.example_id.to_string(),
            example_many_to_manys: examples_many_to_many_dtos,
            created_at: example_with_relation.created_at.to_string(),
            updated_at: example_with_relation.updated_at.to_string()
        }
    }

    pub fn dto_to_create_active_model(dto: ExampleWithRelationCreateDto) -> (ExampleWithRelationActiveModel, Vec<ExampleManyToManyActiveModel>) {
        let example_many_to_many_active_models = dto.example_many_to_manys.unwrap().iter().map(|dto| ExampleManyToManyTransformer::dto_to_create_active_model(dto.clone())).collect();
        
        let example_with_relation_active_model = ExampleWithRelationActiveModel {
            id: NotSet,
            example_id: Set(Uuid::parse_str(&dto.example_id.unwrap()).unwrap()),
            created_at: NotSet,
            updated_at: NotSet
        };

        (example_with_relation_active_model, example_many_to_many_active_models)

    }

    pub fn dto_to_update_active_model(dto: ExampleWithRelationUpdateDto, id: &'a str) -> (ExampleWithRelationActiveModel, Vec<ExampleManyToManyActiveModel>) {
        let example_many_to_many_active_models = dto.example_many_to_manys.unwrap().iter().map(|dto| ExampleManyToManyTransformer::dto_to_update_active_model(dto.clone())).collect();
        
        let example_with_relation_active_model = ExampleWithRelationActiveModel {
            id: Set(Uuid::parse_str(id).unwrap()),
            example_id: Set(Uuid::parse_str(&dto.example_id.unwrap()).unwrap()),
            created_at: NotSet,
            updated_at: Set(Utc::now().naive_utc())
        };

        (example_with_relation_active_model, example_many_to_many_active_models)

    }
}