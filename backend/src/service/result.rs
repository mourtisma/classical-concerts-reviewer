use crate::status::ResponseStatus;
use rocket::serde::Serialize;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct SuccessGetManyResult<M> {
    pub status: ResponseStatus,
    pub items: Vec<M>
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct SuccessGetOneResult<M> {
    pub status: ResponseStatus,
    pub item: M
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct SuccessCreateResult<M> {
    pub status: ResponseStatus,
    pub item: M
}

