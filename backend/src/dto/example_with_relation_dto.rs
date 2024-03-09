use serde::Serialize;

use super::example_many_to_many_dto::ExampleManyToManyGetDto;

#[derive(Clone, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ExampleWithRelationGetDto {
    pub id: String,
    pub example_many_to_manys: Vec<ExampleManyToManyGetDto>,
    pub created_at: String,
    pub updated_at: String,
}