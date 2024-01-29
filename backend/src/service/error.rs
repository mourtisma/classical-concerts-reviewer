use rocket::{serde::Serialize, http::Status};
use crate::{repository::error::RepositoryError, repository::error::RepositoryErrorType, status::ResponseStatus};

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ErrorResult {
    pub status: ResponseStatus,
    pub message: String
}

pub trait ApiError {
    fn new(message: Option<String>) -> Self where Self:Sized;
    fn http_status(&self) -> Status;
    fn to_result(&self) -> ErrorResult;
}

#[derive(Clone)]
pub struct NotFoundError {
    http_status: Status,
    message: String
}


impl<'a> ApiError for NotFoundError {
    fn new(message: Option<String>) -> Self {
        NotFoundError {
            http_status: Status::NotFound,
            message: message.unwrap_or(String::from("Record was not found"))
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

pub struct UnknownError {
    http_status: Status,
    message: String
}


impl ApiError for UnknownError {
    fn new(message: Option<String>) -> Self {
        UnknownError {
            http_status: Status::InternalServerError,
            message: message.unwrap_or(String::from("Unknown error"))
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
        RepositoryErrorType::NotFound => Box::new(NotFoundError::new(rep_error.message)),
        RepositoryErrorType::Unknown => Box::new(UnknownError::new(rep_error.message)),
    }
}
