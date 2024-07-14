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

#[cfg(test)]
mod tests {

    use example_many_to_many_dto::{ExampleManyToManyCreateDto, ExampleManyToManyUpdateDto};
    use rocket::{form::validate::Len, serde::json::to_string};

    use crate::{dto::list_options_dto::ListOptionsDto, model::example_many_to_many};

    use super::*;

    #[test]
    fn test_entity_to_get_dto() {
        let entity = ExampleSeaOrmWithRelationModel {
            id: Uuid::new_v4(),
            example_id: Uuid::new_v4(),
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc()
        };

        let example_many_to_manys = vec![
            ExampleManyToManyModel {
                id: Uuid::new_v4(),
                name: String::from("Example 1"),
                created_at: Utc::now().naive_utc(),
                updated_at: Utc::now().naive_utc()
            },
            ExampleManyToManyModel {
                id: Uuid::new_v4(),
                name: String::from("Example 2"),
                created_at: Utc::now().naive_utc(),
                updated_at: Utc::now().naive_utc()
            }
        ];

        let get_dto = ExampleWithRelationTransformer::entity_to_get_dto((entity.clone(), example_many_to_manys.clone()));
        
        assert_eq!(get_dto.id, entity.id.to_string());
        assert_eq!(get_dto.example_id, entity.example_id.to_string());
        assert_eq!(get_dto.example_many_to_manys.len(), 2);
        assert_eq!(get_dto.created_at, entity.created_at.to_string());

        assert_eq!(get_dto.example_many_to_manys[0].id, example_many_to_manys[0].id.to_string());
        assert_eq!(get_dto.example_many_to_manys[0].name, example_many_to_manys[0].name);
        assert_eq!(get_dto.example_many_to_manys[0].created_at, example_many_to_manys[0].created_at.to_string());
        assert_eq!(get_dto.example_many_to_manys[1].id, example_many_to_manys[1].id.to_string());
        assert_eq!(get_dto.example_many_to_manys[1].name, example_many_to_manys[1].name);
        assert_eq!(get_dto.example_many_to_manys[1].created_at, example_many_to_manys[1].created_at.to_string());
    }

    #[test]
    fn test_dto_to_create_active_model() {
        let create_dto = ExampleWithRelationCreateDto {
            example_id: Some(Uuid::new_v4().to_string()),
            example_many_to_manys: Some(vec![
                ExampleManyToManyCreateDto {
                    name: Some(String::from("ExampleMTM 1"))
                },
                ExampleManyToManyCreateDto {
                    name: Some(String::from("ExampleMTM 2"))
                }
            ])
        };

        let (create_active_model, example_many_to_many_create_active_models) = ExampleWithRelationTransformer::dto_to_create_active_model(create_dto.clone());
        
        let example_many_to_many_dtos = create_dto.example_many_to_manys.clone();

        assert_eq!(create_active_model.id, NotSet);
        assert_eq!(create_active_model.created_at, NotSet);
        assert_eq!(create_active_model.updated_at, NotSet);

        assert_eq!(example_many_to_many_create_active_models.len(), 2);
        assert_eq!(example_many_to_many_create_active_models[0].id, NotSet);
        assert_eq!(example_many_to_many_create_active_models[0].name, Set(example_many_to_many_dtos.clone().unwrap()[0].name.clone().unwrap()));
        assert_eq!(example_many_to_many_create_active_models[0].created_at, NotSet);
        assert_eq!(example_many_to_many_create_active_models[0].updated_at, NotSet);
        assert_eq!(example_many_to_many_create_active_models[1].id, NotSet);
        assert_eq!(example_many_to_many_create_active_models[1].name, Set(example_many_to_many_dtos.unwrap()[1].name.clone().unwrap()));
        assert_eq!(example_many_to_many_create_active_models[1].created_at, NotSet);
        assert_eq!(example_many_to_many_create_active_models[1].updated_at, NotSet);
    }

    #[test]
    fn test_dto_to_update_active_model() {
        let update_dto = ExampleWithRelationUpdateDto {
            example_id: Some(Uuid::new_v4().to_string()),
            example_many_to_manys: Some(vec![
                ExampleManyToManyUpdateDto {
                    id: Some(Uuid::new_v4().to_string()),
                    name: Some(String::from("ExampleMTM 1"))
                },
                ExampleManyToManyUpdateDto {
                    id: None,
                    name: Some(String::from("ExampleMTM 2"))
                }
            ])
        };

        let binding = Uuid::new_v4().to_string();
        let id = binding.as_str();

        let (update_active_model, example_many_to_many_update_active_models) = ExampleWithRelationTransformer::dto_to_update_active_model(update_dto.clone(), id);
        
        assert_eq!(update_active_model.id, Set(Uuid::parse_str(id).unwrap()));
        assert_eq!(update_active_model.created_at, NotSet);

        assert_eq!(example_many_to_many_update_active_models.len(), 2);
        assert_eq!(example_many_to_many_update_active_models[0].id, Set(Uuid::parse_str(&update_dto.clone().example_many_to_manys.unwrap()[0].id.clone().unwrap()).unwrap()));
        assert_eq!(example_many_to_many_update_active_models[0].name, Set(update_dto.clone().example_many_to_manys.unwrap()[0].name.clone().unwrap()));
        assert_eq!(example_many_to_many_update_active_models[0].created_at, NotSet);
        assert_ne!(example_many_to_many_update_active_models[0].updated_at, NotSet);
        assert_eq!(example_many_to_many_update_active_models[1].id, NotSet);
        assert_eq!(example_many_to_many_update_active_models[1].name, Set(update_dto.clone().example_many_to_manys.unwrap()[1].name.clone().unwrap()));
        assert_eq!(example_many_to_many_update_active_models[1].created_at, NotSet);
        assert_ne!(example_many_to_many_update_active_models[1].updated_at, NotSet);
    }

}