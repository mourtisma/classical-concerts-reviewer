use crate::{dto::example_many_to_many_dto::ExampleManyToManyGetDto, model::prelude::ExampleManyToManyModel};

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
}