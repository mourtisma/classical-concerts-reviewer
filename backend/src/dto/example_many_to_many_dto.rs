use rocket::form::validate;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Clone, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ExampleManyToManyGetDto {
    pub id: String,
    pub name: String,
    pub created_at: String,
    pub updated_at: String
}

#[derive(Clone, Serialize, Deserialize, Validate)]
#[serde(crate = "rocket::serde")]
pub struct ExampleManyToManyCreateDto {
    #[validate(required)]
    pub name: Option<String>
}

#[derive(Clone, Serialize, Deserialize, Validate)]
#[serde(crate = "rocket::serde")]
pub struct ExampleManyToManyUpdateDto {
    pub id: Option<String>,
    
    #[validate(required)]
    pub name: Option<String>
}