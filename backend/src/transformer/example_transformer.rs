use std::collections::HashMap;

use chrono::Utc;
use sea_orm::{prelude::*, ActiveValue::NotSet, Order, Set};
use uuid::Uuid;

use crate::{dto::{example_dto::{ExampleCreateDto, ExampleGetDto, ExampleOrderDto, ExampleUpdateDto}, list_options_dto::{ListOptionsDto, OrderType}}, model::{example_sea_orm, prelude::{ExampleActiveModel, ExampleSeaOrm, ExampleSeaOrmModel}, sea_orm_search_params::SeaOrmSearchParams}};

use super::{helpers::order_dto_to_sea_orm, sea_orm_transformer::SeaOrmTransformer};

pub struct ExampleTransformer {}



impl<'a> SeaOrmTransformer<'a, ExampleGetDto, ExampleCreateDto, ExampleUpdateDto, ExampleOrderDto, ExampleSeaOrm, ExampleActiveModel>
 for ExampleTransformer {
    fn entity_to_get_dto(entity: ExampleSeaOrmModel) -> ExampleGetDto {
        ExampleGetDto {
            id: entity.id.to_string(),
            name: entity.name,
            created_at: entity.created_at.to_string()
        }
    }

    fn dto_to_create_active_model(dto: ExampleCreateDto) -> ExampleActiveModel {
        ExampleActiveModel {
            id: NotSet,
            name: Set(dto.name.unwrap()),
            created_at: NotSet,
            updated_at: NotSet,
        }
    }

    fn dto_to_update_active_model(dto: ExampleUpdateDto, id: &'a str) -> ExampleActiveModel {
        ExampleActiveModel {
            id: Set(Uuid::parse_str(id).unwrap()),
            name: Set(dto.name.unwrap()),
            created_at: NotSet,
            updated_at: Set(Utc::now().naive_utc())
        }
    }

    fn active_model_to_dto(active_model: ExampleSeaOrmModel) -> ExampleGetDto {
        ExampleGetDto {
            id: active_model.id.to_string(),
            name: active_model.name,
            created_at: active_model.created_at.to_string()
        }
    }

    fn col_names_to_cols() ->  HashMap<String, <example_sea_orm::Entity as sea_orm::EntityTrait>::Column> {
        let mut cols_map = HashMap::new();
        cols_map.insert("id".to_owned(), example_sea_orm::Column::Id);
        cols_map.insert("name".to_owned(), example_sea_orm::Column::Name);
        cols_map.insert("created_at".to_owned(), example_sea_orm::Column::CreatedAt);
        cols_map.insert("updated_at".to_owned(), example_sea_orm::Column::UpdatedAt);

        cols_map
    }

}

#[cfg(test)]
mod tests {

    use rocket::form::validate::Len;

    use super::*;

    #[test]
    fn test_entity_to_get_dto() {
        let entity = ExampleSeaOrmModel {
            id: Uuid::new_v4(),
            name: String::from("Example 1"),
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc()
        };

        let get_dto = ExampleTransformer::entity_to_get_dto(entity.clone());
        
        assert_eq!(get_dto.id, entity.id.to_string());
        assert_eq!(get_dto.name, entity.name);
        assert_eq!(get_dto.created_at, entity.created_at.to_string());
    }

    #[test]
    fn test_dto_to_create_active_model() {
        let create_dto = ExampleCreateDto {
            name: Some(String::from("Example 1"))
        };

        let create_active_model = ExampleTransformer::dto_to_create_active_model(create_dto.clone());
        
        assert_eq!(create_active_model.id, NotSet);
        assert_eq!(create_active_model.name, Set(create_dto.name.unwrap()));
        assert_eq!(create_active_model.created_at, NotSet);
        assert_eq!(create_active_model.updated_at, NotSet);
    }

    #[test]
    fn test_dto_to_update_active_model() {
        let update_dto = ExampleUpdateDto {
            name: Some(String::from("Example 1"))
        };

        let binding = Uuid::new_v4().to_string();
        let id = binding.as_str();

        let update_active_model = ExampleTransformer::dto_to_update_active_model(update_dto.clone(), id);
        
        assert_eq!(update_active_model.id, Set(Uuid::parse_str(id).unwrap()));
        assert_eq!(update_active_model.name, Set(update_dto.name.unwrap()));
        assert_eq!(update_active_model.created_at, NotSet);
    }

    #[test]
    fn test_active_model_to_dto() {
        let entity = ExampleSeaOrmModel {
            id: Uuid::new_v4(),
            name: String::from("Example 1"),
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc()
        };

        let get_dto = ExampleTransformer::entity_to_get_dto(entity.clone());
        
        assert_eq!(get_dto.id, entity.id.to_string());
        assert_eq!(get_dto.name, entity.name);
        assert_eq!(get_dto.created_at, entity.created_at.to_string());
    }

    #[test]
    fn test_list_options_to_search_params() {
        let list_options =  ListOptionsDto {
            order_by: Some(vec![
                ExampleOrderDto {
                    field: String::from("name"),
                    direction: OrderType::Asc
                },
                ExampleOrderDto {
                    field: String::from("created_at"),
                    direction: OrderType::Desc
                }
                ]),
                page: Some(1),
                limit: Some(2)
        };

        let search_params = ExampleTransformer::list_options_to_search_params::<ExampleTransformer>(list_options.clone());

        assert!(search_params.order_by.is_some());
        assert_eq!(search_params.order_by.len(), 2);
        
        assert!(matches!(search_params.order_by.clone().unwrap()[0], (example_sea_orm::Column::Name, Order::Asc)));
        assert!(matches!(search_params.order_by.clone().unwrap()[1], (example_sea_orm::Column::CreatedAt, Order::Desc)));

        assert_eq!(search_params.page_number, list_options.clone().page);
        assert_eq!(search_params.page_size, list_options.clone().limit);
    }
}