use rocket_db_pools::{diesel::{PgPool, prelude::*}, Database};

#[derive(Database)]
#[database("ccr")]
pub struct Ccr(PgPool);