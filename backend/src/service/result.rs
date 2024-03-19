use crate::status::ResponseStatus;
use rocket::serde::Serialize;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct SuccessGetManyResult<M> {
    pub status: ResponseStatus,
    pub items: Vec<M>,
    pub total_count: u64,
    pub num_pages: Option<u64>
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

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct SuccessUpdateResult<M> {
    pub status: ResponseStatus,
    pub item: M
}

