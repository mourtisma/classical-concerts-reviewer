use crate::model::base_model::BaseModel;

use super::list_options::ListOptions;

pub trait BaseRepository<M> where M: BaseModel {
    fn new() -> Self where Self: Sized;
    fn get_many(&self, options: ListOptions) -> Vec<M>;
    fn get_one(&self, id: &str) -> Option<M>;
    fn create(&mut self, data: M) -> M;
}