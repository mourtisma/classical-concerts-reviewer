// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "citext"))]
    pub struct Citext;
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::Citext;

    examples (id) {
        id -> VarChar,
        name -> VarChar
    }
}
