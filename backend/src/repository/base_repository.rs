use crate::model::base_model::BaseModel;

use super::{error::RepositoryError, list_options::ListOptions};

pub trait BaseRepository<M> where M: BaseModel {
    fn new() -> Self where Self: Sized;
    fn get_many(&self, options: ListOptions) -> Vec<M>;
    fn get_one(&self, id: &str) -> Option<M>;
    fn create(&mut self, data: M) -> M;
    fn update(&mut self, id: &str, data: M) -> Result<M, RepositoryError>;
    fn delete<'a>(&mut self, id: &'a str) -> Result<(), RepositoryError>;
}