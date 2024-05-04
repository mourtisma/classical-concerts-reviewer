use validator::Validate;

use super::list_options_dto::OrderType;

pub trait OrderDto: Validate + Clone {
    fn field(self) -> String;
    fn direction(self) -> OrderType;
}