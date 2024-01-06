use crate::{model::base_model::BaseModel, repository::{base_repository::BaseRepository, list_options::ListOptions}};

pub struct BaseService<M> where M: BaseModel {
    pub repository: Box<dyn BaseRepository<M>>
}

impl<M> BaseService<M> where M: BaseModel {
    pub fn get_many(&mut self, options: ListOptions) -> &Vec<M> {
        self.repository.get_many(options)
    }
}