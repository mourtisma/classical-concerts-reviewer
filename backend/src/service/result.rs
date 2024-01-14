use crate::status::status;
use rocket::serde::Serialize;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct SuccessGetManyResult<M> {
    pub status: status,
    pub items: Vec<M>
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct SuccessGetOneResult<M> {
    pub status: status,
    pub item: M
}

