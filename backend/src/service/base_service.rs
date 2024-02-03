use std::borrow::BorrowMut;

use rocket::futures::future::err;
use validator::ValidationErrors;

use crate::{model::base_model::BaseModel, repository::{base_repository::BaseRepository, list_options::ListOptions}, status::ResponseStatus};

use super::{result::{SuccessCreateResult, SuccessGetManyResult, SuccessGetOneResult, SuccessUpdateResult}, error::{to_api_error, ApiError, ApiValidationError, NotFoundError}};

pub struct BaseService<'a, M> where M: BaseModel<'a> {
    pub repository: Box<dyn BaseRepository<'a, M> + 'a>,
}

impl<'a, M> BaseService<'a, M> where M: BaseModel<'a> {
    pub fn get_many(&self, options: ListOptions) -> SuccessGetManyResult<M> {
        let items = self.repository.get_many(options);

        SuccessGetManyResult {
            status: ResponseStatus::Success,
            items
        }
    }

    pub fn get_one(&self, id: &str) -> Result<SuccessGetOneResult<M>, impl ApiError<'a> + Copy> {
        if let Some(item) = self.repository.get_one(id) {
            Ok(SuccessGetOneResult {
                status: ResponseStatus::Success,
                item
            })
        } else {
            Err(NotFoundError::new(None, None))
        }
        
    }

    pub fn create(&mut self, data: M) -> Result<SuccessCreateResult<M>, impl ApiError<'a>> {
        let validation_result = data.validate();
        if validation_result.is_err() {
            return Err(ApiValidationError::new(None, validation_result.err()))
        }

        let item = self.repository.create(data);

        Ok(SuccessCreateResult {
            status: ResponseStatus::Success,
            item
        })
    }

    pub fn update(&mut self, id: &'a str, data: M) -> Result<SuccessUpdateResult<M>, Box<dyn ApiError<'a> + 'a>> {
        let update_result = self.repository.update(id, data);

        match update_result {
            Err(rep_err) => Err(to_api_error(rep_err)),
            Ok(item) => Ok(SuccessUpdateResult {
                status: ResponseStatus::Success,
                item
            })
        }
    }

    pub fn delete(&mut self, id: &'a str) -> Result<(), Box<dyn ApiError<'a> + 'a>> {
        let delete_result = self.repository.delete(id);

        match delete_result {
            Err(rep_err) => Err(to_api_error(rep_err)),
            Ok(()) => Ok(())
        }
    }
}