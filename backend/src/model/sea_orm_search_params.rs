use sea_orm::{ColumnTrait, IntoSimpleExpr, Order};

pub struct SeaOrmSearchParams<C> where C: IntoSimpleExpr {
    pub order_by: Option<Vec<(C, Order)>>,
    pub page_number: Option<u64>,
    pub page_size: Option<u64>,
}