use crate::status::status;
use rocket::serde::Serialize;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct SuccessGetManyResult<M> {
    pub status: status,
    pub items: Vec<M>
}