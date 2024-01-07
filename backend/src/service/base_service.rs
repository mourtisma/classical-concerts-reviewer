use crate::{model::base_model::BaseModel, repository::{base_repository::BaseRepository, list_options::ListOptions}, status::status};

use super::result::SuccessGetManyResult;

pub struct BaseService<M> where M: BaseModel {
    pub repository: Box<dyn BaseRepository<M>>,
}

impl<M> BaseService<M> where M: BaseModel {
    pub fn get_many(&self, options: ListOptions) -> SuccessGetManyResult<M> {
        let items = self.repository.get_many(options);

        SuccessGetManyResult {
            status: status::success,
            items: items
        }
    }
}