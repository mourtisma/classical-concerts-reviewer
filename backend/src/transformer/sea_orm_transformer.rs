use std::collections::HashMap;

use sea_orm::{ActiveModelBehavior, ColumnTrait, EntityTrait, Order};
use uuid::Uuid;

use crate::{dto::list_options_dto::{ListOptionsDto, OrderDto, OrderType}, model::sea_orm_search_params::SeaOrmSearchParams};

pub trait SeaOrmTransformer<'a, GetDto, CreateDto, UpdateDto, E: sea_orm::EntityTrait, AM: sea_orm::ActiveModelTrait> {
    fn entity_to_get_dto(entity: <E as sea_orm::EntityTrait>::Model) -> GetDto;
    fn dto_to_create_active_model(dto: CreateDto) -> AM;
    fn dto_to_update_active_model(dto: UpdateDto, id: &'a str) -> AM;
    fn active_model_to_dto(active_model: <<AM as sea_orm::ActiveModelTrait>::Entity as sea_orm::EntityTrait>::Model) -> GetDto;

    fn list_options_to_search_params(list_options: ListOptionsDto) -> SeaOrmSearchParams<E::Column>;
    fn col_names_to_cols() ->  HashMap<String, E::Column>;
    fn build_order_vec(list_options_order: Option<Vec<OrderDto>>) -> Option<Vec<(E::Column, Order)>>;
}
