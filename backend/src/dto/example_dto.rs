use rocket::form::FromForm;
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};

use super::list_options_dto::OrderType;

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

fn validate_field(value: &String) -> Result<(), ValidationError> {
    let sortable_fields = vec!["name", "created_at", "updated_at"];
    if !sortable_fields.contains(&value.as_str()) {
        Err(ValidationError::new("Invalid field name"))
    } else {
        Ok(())
    }
}

#[derive(FromForm, Validate)]
pub struct ExampleOrderDto {
    #[validate(custom(function = "validate_field"))]
    pub field: String,
    #[validate(custom(function = "super::list_options_dto::validate_direction"))]
    pub direction: OrderType
}