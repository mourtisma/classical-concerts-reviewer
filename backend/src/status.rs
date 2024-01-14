use rocket::serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "lowercase")]
#[serde(crate = "rocket::serde")]
pub enum ResponseStatus {
    Success,
    Error
}