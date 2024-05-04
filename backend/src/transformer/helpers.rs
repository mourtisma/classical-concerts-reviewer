use sea_orm::Order;

use crate::dto::list_options_dto::OrderType;

pub fn order_dto_to_sea_orm(direction: OrderType) -> Order {
    match direction {
        OrderType::Asc => Order::Asc,
        OrderType::Desc => Order::Desc
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_order_dto_to_sea_orm() {
        assert_eq!(Order::Asc, order_dto_to_sea_orm(OrderType::Asc));
        assert_eq!(Order::Desc, order_dto_to_sea_orm(OrderType::Desc));
    }
}