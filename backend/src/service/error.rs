use rocket::{serde::Serialize, http::Status};
use crate::status::ResponseStatus;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ErrorResult {
    pub status: ResponseStatus,
    pub message: String
}

pub trait ApiError {
    fn new() -> Self;
    fn http_status(self) -> Status;
    fn to_result(self) -> ErrorResult;
}

#[derive(Clone, Copy)]
pub struct NotFoundError<'a> {
    http_status: Status,
    message: &'a str
}


impl ApiError for NotFoundError<'_> {
    fn new() -> Self {
        NotFoundError {
            http_status: Status::NotFound,
            message: "Record was not found"
        }
    }

    fn http_status(self) -> Status {
        self.http_status
    }

    fn to_result(self) -> ErrorResult {
        ErrorResult {
            status: ResponseStatus::Error,
            message: self.message.to_string(),
        }
    }
}
