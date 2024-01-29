use rocket::{serde::Serialize, http::Status};
use crate::{repository::error::RepositoryError, repository::error::RepositoryErrorType, status::ResponseStatus};

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ErrorResult<'a> {
    pub status: ResponseStatus,
    pub message: &'a str
}

pub trait ApiError<'a> {
    fn new(message: Option<&'a str>) -> Self where Self:Sized;
    fn http_status(&self) -> Status;
    fn to_result(&self) -> ErrorResult<'a>;
}

#[derive(Clone, Copy)]
pub struct NotFoundError<'a> {
    http_status: Status,
    message: &'a str
}


impl<'a> ApiError<'a> for NotFoundError<'a> {
    fn new(message: Option<&'a str>) -> Self {
        NotFoundError {
            http_status: Status::NotFound,
            message: message.unwrap_or("Record was not found")
        }
    }

    fn http_status(&self) -> Status {
        self.http_status
    }

    fn to_result(&self) -> ErrorResult<'a> {
        ErrorResult {
            status: ResponseStatus::Error,
            message: self.message,
        }
    }
}

#[derive(Clone, Copy)]
pub struct UnknownError<'a> {
    http_status: Status,
    message: &'a str
}


impl<'a> ApiError<'a> for UnknownError<'a> {
    fn new(message: Option<&'a str>) -> Self {
        UnknownError {
            http_status: Status::InternalServerError,
            message: message.unwrap_or("Unknown error")
        }
    }

    fn http_status(&self) -> Status {
        self.http_status
    }

    fn to_result(&self) -> ErrorResult<'a> {
        ErrorResult {
            status: ResponseStatus::Error,
            message: self.message,
        }
    }
}

pub fn to_api_error<'a>(rep_error: RepositoryError<'a>) -> Box<dyn ApiError<'a> + 'a> {
    match rep_error.error_type {
        RepositoryErrorType::NotFound => Box::new(NotFoundError::new(rep_error.message)),
        RepositoryErrorType::Unknown => Box::new(UnknownError::new(rep_error.message)),
    }
}
