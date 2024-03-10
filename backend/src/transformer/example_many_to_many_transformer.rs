use sea_orm::{ActiveValue::NotSet, Set};

use crate::{dto::example_many_to_many_dto::{ExampleManyToManyCreateDto, ExampleManyToManyGetDto}, model::prelude::{ExampleManyToManyModel, ExampleManyToManyActiveModel}};

pub struct ExampleManyToManyTransformer {}

impl ExampleManyToManyTransformer {
    pub fn entity_to_get_dto(entity: ExampleManyToManyModel) -> ExampleManyToManyGetDto {
        ExampleManyToManyGetDto {
            id: entity.id.to_string(),
            name: entity.name,
            created_at: entity.created_at.to_string(),
            updated_at: entity.updated_at.to_string()
        }
    }

    pub fn dto_to_create_active_model(dto: ExampleManyToManyCreateDto) -> ExampleManyToManyActiveModel {
        ExampleManyToManyActiveModel {
            id: NotSet,
            name: Set(dto.name.unwrap()),
            created_at: NotSet,
            updated_at: NotSet
        }
    }
}