use crate::{model::example::Example, repository::{example_pg_repository::ExamplePgRepository, list_options::ListOptions}, status::ResponseStatus};

use super::{error::{to_api_error, ApiError, NotFoundError, UnknownError}, result::{SuccessGetManyResult, SuccessGetOneResult}};

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
}