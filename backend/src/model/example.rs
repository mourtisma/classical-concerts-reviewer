use diesel::{expression::AsExpression, Selectable};
use rocket::{serde::{Deserialize, Serialize}};
use rocket_db_pools::diesel::{Queryable, Insertable};
use uuid::Uuid;
use validator::Validate;

use crate::schema::sql_types::Citext;

use super::base_model::BaseModel;

#[derive(Clone, Serialize, Deserialize, Queryable, Selectable)]
#[diesel(table_name = crate::schema::examples)]
#[serde(crate = "rocket::serde")]
pub struct Example {
    pub id: String,
    pub name: String
}

impl BaseModel for Example {
    fn id(self) -> String {
        self.id
    }

}

#[derive(Clone, Deserialize, Insertable, Validate)]
#[diesel(table_name = crate::schema::examples)]
#[serde(crate = "rocket::serde")]
pub struct ExampleSave {

    #[validate(required)]
    pub name: Option<String>
}
