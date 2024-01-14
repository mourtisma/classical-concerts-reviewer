use crate::{model::base_model::BaseModel, repository::{base_repository::BaseRepository, list_options::ListOptions}, status::ResponseStatus};

use super::{result::{SuccessGetManyResult, SuccessGetOneResult}, error::{NotFoundError, ApiError}};

pub struct BaseService<M> where M: BaseModel {
    pub repository: Box<dyn BaseRepository<M>>,
}

impl<M> BaseService<M> where M: BaseModel {
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
}