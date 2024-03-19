use std::collections::HashMap;

use rocket::{form::{validate, FromForm}, FromFormField};
use serde::Serialize;
use validator::{Validate, ValidationError};

pub fn validate_direction(value: &OrderType) -> Result<(), ValidationError> {
    match value {
        OrderType::Asc => Ok(()),
        OrderType::Desc => Ok(()),
        _ => Err(ValidationError::new("Invalid order"))
    }
}

#[derive(Clone, FromFormField, Serialize)]
pub enum OrderType {
    Asc,
    Desc
}

#[derive(Validate)]
pub struct ListOptionsDto<EntityOrderDto> where EntityOrderDto: Validate {
    #[validate]
    pub order_by: Option<Vec<EntityOrderDto>>,
    #[validate(range(min = 1))]
    pub page: Option<u64>,
    #[validate(range(min = 1))]
    pub limit: Option<u64>
}