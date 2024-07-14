use chrono::Utc;
use sea_orm::{ActiveValue::NotSet, Set};
use uuid::Uuid;

use crate::{dto::example_many_to_many_dto::{ExampleManyToManyCreateDto, ExampleManyToManyGetDto, ExampleManyToManyUpdateDto}, model::prelude::{ExampleManyToManyActiveModel, ExampleManyToManyModel}};

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

    pub fn dto_to_update_active_model(dto: ExampleManyToManyUpdateDto) -> ExampleManyToManyActiveModel {
        let id = match dto.id {
            Some(x) => Set(Uuid::parse_str(&x).unwrap()),
            _ => NotSet
        };

        ExampleManyToManyActiveModel {
            id,
            name: Set(dto.name.unwrap()),
            created_at: NotSet,
            updated_at: Set(Utc::now().naive_utc())
        }
    }
}

#[cfg(test)]
mod tests {

    use rocket::form::validate::Len;

    use super::*;

    #[test]
    fn test_entity_to_get_dto() {
        let entity = ExampleManyToManyModel {
            id: Uuid::new_v4(),
            name: String::from("ExampleMTM 1"),
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc()
        };

        let get_dto = ExampleManyToManyTransformer::entity_to_get_dto(entity.clone());
        
        assert_eq!(get_dto.id, entity.id.to_string());
        assert_eq!(get_dto.name, entity.name);
        assert_eq!(get_dto.created_at, entity.created_at.to_string());
    }

    #[test]
    fn test_dto_to_create_active_model() {
        let create_dto = ExampleManyToManyCreateDto {
            name: Some(String::from("ExampleMTM 1"))
        };

        let create_active_model = ExampleManyToManyTransformer::dto_to_create_active_model(create_dto.clone());
        
        assert_eq!(create_active_model.id, NotSet);
        assert_eq!(create_active_model.name, Set(create_dto.name.unwrap()));
        assert_eq!(create_active_model.created_at, NotSet);
        assert_eq!(create_active_model.updated_at, NotSet);
    }

    #[test]
    fn test_dto_to_update_active_model() {
        let update_dto = ExampleManyToManyUpdateDto {
            id: Some(Uuid::new_v4().to_string()),
            name: Some(String::from("ExampleMTM 1"))
        };


        let update_active_model = ExampleManyToManyTransformer::dto_to_update_active_model(update_dto.clone());
        
        assert_eq!(update_active_model.id, Set(Uuid::parse_str(&update_dto.clone().id.unwrap()).unwrap()));
        assert_eq!(update_active_model.name, Set(update_dto.name.unwrap()));
        assert_eq!(update_active_model.created_at, NotSet);
    }

}