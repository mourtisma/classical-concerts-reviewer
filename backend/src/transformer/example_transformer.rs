use chrono::Utc;
use sea_orm::{prelude::*, ActiveValue::NotSet, Set};
use uuid::Uuid;

use crate::{dto::example_dto::{ExampleCreateDto, ExampleGetDto, ExampleUpdateDto}, model::{example_sea_orm, prelude::{ExampleActiveModel, ExampleSeaOrm, ExampleSeaOrmModel}}};

use super::sea_orm_transformer::SeaOrmTransformer;

pub struct ExampleTransformer {}

impl<'a> SeaOrmTransformer<'a, ExampleGetDto, ExampleCreateDto, ExampleUpdateDto, ExampleSeaOrm, ExampleActiveModel>
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
}