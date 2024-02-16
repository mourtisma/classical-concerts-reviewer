use crate::{model::example::Example, repository::{example_pg_repository::ExamplePgRepository, list_options::ListOptions}, status::ResponseStatus};

use super::result::SuccessGetManyResult;

pub struct ExampleService<'a> {
    pub repository: ExamplePgRepository<'a>
}

impl<'a> ExampleService<'a> {
    pub async fn get_many(&mut self, options: ListOptions) -> SuccessGetManyResult<Example> {
        let items = self.repository.get_many(options).await;

        SuccessGetManyResult {
            status: ResponseStatus::Success,
            items
        }
    }
}