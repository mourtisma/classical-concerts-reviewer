use rocket::form::validate;
use serde::{Deserialize, Serialize};
use validator::Validate;

use super::example_many_to_many_dto::{ExampleManyToManyCreateDto, ExampleManyToManyGetDto, ExampleManyToManyUpdateDto};

#[derive(Clone, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ExampleWithRelationGetDto {
    pub id: String,
    pub example_id: String,
    pub example_many_to_manys: Vec<ExampleManyToManyGetDto>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Clone, Deserialize, Validate)]
#[serde(crate = "rocket::serde")]
pub struct ExampleWithRelationCreateDto {
    #[validate(required)]
    pub example_id: Option<String>,
    #[validate]
    #[validate(required, length(min = 1))]
    pub example_many_to_manys: Option<Vec<ExampleManyToManyCreateDto>>
}

#[derive(Clone, Deserialize, Validate)]
#[serde(crate = "rocket::serde")]
pub struct ExampleWithRelationUpdateDto {
    #[validate(required)]
    pub example_id: Option<String>,
    #[validate]
    #[validate(required, length(min = 1))]
    pub example_many_to_manys: Option<Vec<ExampleManyToManyUpdateDto>>
}