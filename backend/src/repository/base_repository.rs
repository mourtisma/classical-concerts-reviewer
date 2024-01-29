use crate::model::base_model::BaseModel;

use super::{error::RepositoryError, list_options::ListOptions};

pub trait BaseRepository<'a, M> where M: BaseModel<'a> {
    fn new() -> Self where Self: Sized;
    fn get_many(&self, options: ListOptions) -> Vec<M>;
    fn get_one(&self, id: &str) -> Option<M>;
    fn create(&mut self, data: M) -> M;
    fn update(&mut self, id: &str, data: M) -> Result<M, RepositoryError<'a>>;
    fn delete(&mut self, id: &'a str) -> Result<(), RepositoryError<'a>>;
}