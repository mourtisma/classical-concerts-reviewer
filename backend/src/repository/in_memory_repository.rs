use crate::model::base_model::BaseModel;

use super::{base_repository::BaseRepository, list_options::ListOptions};

pub struct InMemoryRepository<M> where M: BaseModel {
    data: Vec<M>
}

impl<M> BaseRepository<M> for InMemoryRepository<M> where M: BaseModel {
    fn new() -> Self where Self: Sized {
       let data = M::populate_data();
       let repository = InMemoryRepository { data: data };

       repository
    }

    fn get_many(&self, options: ListOptions) -> Vec<M> {
        self.data.clone()    
    }

    fn get_one(&self, id: &str) -> Option<M> {
        let data = self.data.clone();

        data.iter().find(|&&x| x.id() == id).cloned()
 
    }
}