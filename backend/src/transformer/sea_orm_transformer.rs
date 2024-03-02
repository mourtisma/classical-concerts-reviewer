use sea_orm::{ActiveModelBehavior, EntityTrait};
use uuid::Uuid;

pub trait SeaOrmTransformer<'a, GetDto, CreateDto, UpdateDto, E: sea_orm::EntityTrait, AM: sea_orm::ActiveModelTrait> {
    fn entity_to_get_dto(entity: <E as sea_orm::EntityTrait>::Model) -> GetDto;
    fn dto_to_create_active_model(dto: CreateDto) -> AM;
    fn dto_to_update_active_model(dto: UpdateDto, id: &'a str) -> AM;
    fn active_model_to_dto(active_model: <<AM as sea_orm::ActiveModelTrait>::Entity as sea_orm::EntityTrait>::Model) -> GetDto;
}
