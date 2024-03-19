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

    fn build_order_vec(list_options_order: Option<Vec<ExampleOrderDto>>) -> Option<Vec<(<example_sea_orm::Entity as sea_orm::EntityTrait>::Column, Order)>> {
        if let Some(order_options) = list_options_order {
            let mut order_vec = vec![];

            let cols_map = ExampleTransformer::col_names_to_cols();
    
            for order_dto in order_options.iter() {
                let sea_orm_col = cols_map.get(&order_dto.field).unwrap(); 
                let sea_orm_order = order_dto_to_sea_orm(order_dto.direction.clone());
    
                order_vec.push((*sea_orm_col, sea_orm_order));
            }
    
            Some(order_vec)
        } else {
            None
        }
        

    }
    
    fn list_options_to_search_params(list_options: ListOptionsDto<ExampleOrderDto>) -> SeaOrmSearchParams<<ExampleSeaOrm as EntityTrait>::Column> {
        SeaOrmSearchParams::<example_sea_orm::Column> {
            order_by: ExampleTransformer::build_order_vec(list_options.order_by),
            page_number: list_options.page,
            page_size: list_options.limit,
        }
    }


}