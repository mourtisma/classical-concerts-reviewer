use sea_orm::Order;

use crate::dto::list_options_dto::OrderType;

pub fn order_dto_to_sea_orm(direction: OrderType) -> Order {
    match direction {
        OrderType::Asc => Order::Asc,
        OrderType::Desc => Order::Desc
    }
}