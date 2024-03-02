use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Clone, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ExampleGetDto {
    pub id: String,
    pub name: String,
    pub created_at: String
}

#[derive(Clone, Deserialize, Validate)]
#[serde(crate = "rocket::serde")]
pub struct ExampleCreateDto {
    #[validate(required)]
    pub name: Option<String>
}

#[derive(Clone, Deserialize, Validate)]
#[serde(crate = "rocket::serde")]
pub struct ExampleUpdateDto {
    #[validate(required)]
    pub name: Option<String>
}