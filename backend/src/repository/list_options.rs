use std::collections::HashMap;

pub struct ListOptions {
    pub order_by: Option<HashMap<String, String>>,
    pub page: Option<u64>,
    pub limit: Option<u64>
}