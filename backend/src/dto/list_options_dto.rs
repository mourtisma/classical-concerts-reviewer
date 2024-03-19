use std::collections::HashMap;

use rocket::{form::{self, FromForm}, FromFormField};

#[derive(Clone, FromFormField)]
pub enum OrderType {
    Asc,
    Desc
}

#[derive(FromForm)]
pub struct OrderDto {
    #[form(field = "field")]
    pub field: String,
    #[form(field = "direction")]
    pub direction: OrderType
}

pub struct ListOptionsDto {
    pub order_by: Option<Vec<OrderDto>>,
    pub page: Option<u64>,
    pub limit: Option<u64>
}