use serde::Serialize;

#[derive(Clone, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ExampleManyToManyGetDto {
    pub id: String,
    pub name: String,
    pub created_at: String,
    pub updated_at: String
}