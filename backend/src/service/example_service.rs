use crate::{model::example::Example, repository::{example_pg_repository::ExamplePgRepository, list_options::ListOptions}, status::ResponseStatus};

use super::{error::{to_api_error, ApiError}, result::SuccessGetManyResult};

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
}