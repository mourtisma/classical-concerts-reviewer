use rocket::{serde::Serialize, http::Status};
use crate::{repository::error::RepositoryError, repository::error::RepositoryErrorType, status::ResponseStatus};

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ErrorResult {
    pub status: ResponseStatus,
    pub message: String
}

pub trait ApiError {
    fn new() -> Self where Self:Sized;
    fn http_status(&self) -> Status;
    fn to_result(&self) -> ErrorResult;
}

#[derive(Clone, Copy)]
pub struct NotFoundError<'a> {
    http_status: Status,
    message: &'a str
}


impl<'a> ApiError for NotFoundError<'a> {
    fn new() -> Self {
        NotFoundError {
            http_status: Status::NotFound,
            message: "Record was not found"
        }
    }

    fn http_status(&self) -> Status {
        self.http_status
    }

    fn to_result(&self) -> ErrorResult {
        ErrorResult {
            status: ResponseStatus::Error,
            message: self.message.to_string(),
        }
    }
}

pub struct UnknownError<'a> {
    http_status: Status,
    message: &'a str
}


impl<'a> ApiError for UnknownError<'a> {
    fn new() -> Self {
        UnknownError {
            http_status: Status::InternalServerError,
            message: "Unknown error"
        }
    }

    fn http_status(&self) -> Status {
        self.http_status
    }

    fn to_result(&self) -> ErrorResult {
        ErrorResult {
            status: ResponseStatus::Error,
            message: self.message.to_string(),
        }
    }
}

pub fn to_api_error(rep_error: RepositoryError) -> Box<dyn ApiError> {
    match rep_error.error_type {
        RepositoryErrorType::NotFound => Box::new(NotFoundError::new()),
        RepositoryErrorType::Unknown => Box::new(UnknownError::new()),
    }
}
