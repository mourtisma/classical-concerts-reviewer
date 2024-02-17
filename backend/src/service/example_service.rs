use validator::Validate;

use crate::{model::example::{Example, ExampleSave}, repository::{example_pg_repository::ExamplePgRepository, list_options::ListOptions}, status::ResponseStatus};

use super::{error::{to_api_error, ApiError, ApiValidationError, NotFoundError, UnknownError}, result::{SuccessCreateResult, SuccessGetManyResult, SuccessGetOneResult, SuccessUpdateResult}};

pub struct ExampleService<'a> {
    pub repository: ExamplePgRepository<'a>
}

impl<'a> ExampleService<'a> {
    pub async fn get_many(&mut self, options: ListOptions) -> Result<SuccessGetManyResult<Example>, Box<dyn ApiError<'a> + 'a>> {
        let repository_result = self.repository.get_many(options).await;
        
        match repository_result {
            Err(rep_error) => Err(to_api_error(rep_error)),
            Ok(items) => Ok(SuccessGetManyResult {
                status: ResponseStatus::Success,
                items
            })
        }
        
    }

    pub async fn get_one(&mut self, example_id: &str) -> Result<SuccessGetOneResult<Example>, Box<dyn ApiError<'a> + 'a>> {
        let repository_result = self.repository.get_one(example_id).await;
        
        if let Ok(item) = repository_result {
            match item {
                None => Err(Box::new(NotFoundError::new(None, None))),
                Some(item) => Ok(SuccessGetOneResult {
                    status: ResponseStatus::Success,
                    item
                })
            }
        } else if let Err(repository_error) = repository_result {
            return Err(to_api_error(repository_error))
        } else {
            return Err(Box::new(UnknownError::new(None, None)))
        }
        
    }

    pub async fn create(&mut self, data: ExampleSave) -> Result<SuccessCreateResult<Example>, Box<dyn ApiError<'a> + 'a>> {
        let validation_result = data.validate();
        if validation_result.is_err() {
            return Err(Box::new(ApiValidationError::new(None, validation_result.err())))
        }

        let repository_result = self.repository.create(data).await;

        match repository_result {
            Err(rep_error) => Err(to_api_error(rep_error)),
            Ok(item) => Ok(SuccessCreateResult {
                status: ResponseStatus::Success,
                item
            })
        }
        
    }

    pub async fn update(&mut self, id: &'a str, data: ExampleSave) -> Result<SuccessUpdateResult<Example>, Box<dyn ApiError<'a> + 'a>> {
        let validation_result = data.validate();
        if validation_result.is_err() {
            return Err(Box::new(ApiValidationError::new(None, validation_result.err())))
        }

        let repository_result = self.repository.update(id, data).await;

        match repository_result {
            Err(rep_err) => Err(to_api_error(rep_err)),
            Ok(item) => Ok(SuccessUpdateResult {
                status: ResponseStatus::Success,
                item
            })
        }
    }
}