use crate::{model::base_model::BaseModel, repository::{base_repository::BaseRepository, list_options::ListOptions}, status::ResponseStatus};

use super::{result::{SuccessCreateResult, SuccessGetManyResult, SuccessGetOneResult}, error::{NotFoundError, ApiError}};

pub struct BaseService<'a, M> where M: BaseModel {
    pub repository: Box<dyn BaseRepository<M> + 'a>,
}

impl<'a, M> BaseService<'a, M> where M: BaseModel {
    pub fn get_many(&self, options: ListOptions) -> SuccessGetManyResult<M> {
        let items = self.repository.get_many(options);

        SuccessGetManyResult {
            status: ResponseStatus::Success,
            items
        }
    }

    pub fn get_one(&self, id: &str) -> Result<SuccessGetOneResult<M>, impl ApiError + Copy> {
        if let Some(item) = self.repository.get_one(id) {
            Ok(SuccessGetOneResult {
                status: ResponseStatus::Success,
                item
            })
        } else {
            Err(NotFoundError::new())
        }
        
    }

    pub fn create(&mut self, data: M) -> SuccessCreateResult<M> {
        let item = self.repository.create(data);

        SuccessCreateResult {
            status: ResponseStatus::Success,
            item
        }
    }
}