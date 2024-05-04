use std::collections::HashMap;

use sea_orm::{ActiveModelBehavior, ColumnTrait, EntityTrait, Order};
use uuid::Uuid;
use validator::Validate;

use crate::{dto::{list_options_dto::{ListOptionsDto, OrderType}, order_dto::{self, OrderDto}}, model::sea_orm_search_params::SeaOrmSearchParams};

use super::helpers::order_dto_to_sea_orm;

pub trait SeaOrmTransformer<'a, GetDto, CreateDto, UpdateDto, EntityOrderDto, E: sea_orm::EntityTrait, AM: sea_orm::ActiveModelTrait> where EntityOrderDto: Validate {
    fn entity_to_get_dto(entity: <E as sea_orm::EntityTrait>::Model) -> GetDto;
    fn dto_to_create_active_model(dto: CreateDto) -> AM;
    fn dto_to_update_active_model(dto: UpdateDto, id: &'a str) -> AM;
    fn active_model_to_dto(active_model: <<AM as sea_orm::ActiveModelTrait>::Entity as sea_orm::EntityTrait>::Model) -> GetDto;

    fn col_names_to_cols() ->  HashMap<String, E::Column>;
    fn build_order_vec<Transformer>(list_options_order: Option<Vec<EntityOrderDto>>) -> Option<Vec<(E::Column, Order)>> where 
    EntityOrderDto: OrderDto,
    Transformer: SeaOrmTransformer<'a, GetDto, CreateDto, UpdateDto, EntityOrderDto, E, AM> {
        if let Some(order_options) = list_options_order {
            let mut order_vec = vec![];

            let cols_map = Transformer::col_names_to_cols();
    
            for order_dto in order_options.iter() {
                let sea_orm_col = cols_map.get(&order_dto.clone().field()).unwrap(); 
                let sea_orm_order = order_dto_to_sea_orm(order_dto.clone().direction().clone());
    
                order_vec.push((*sea_orm_col, sea_orm_order));
            }
    
            Some(order_vec)
        } else {
            None
        }
        

    }

    fn list_options_to_search_params<Transformer>(list_options: ListOptionsDto<EntityOrderDto>) -> SeaOrmSearchParams<E::Column> where 
    EntityOrderDto: OrderDto,
    Transformer: SeaOrmTransformer<'a, GetDto, CreateDto, UpdateDto, EntityOrderDto, E, AM> {
        SeaOrmSearchParams::<E::Column> {
            order_by: Transformer::build_order_vec::<Transformer>(list_options.order_by),
            page_number: list_options.page,
            page_size: list_options.limit,
        }
    }
    
}
