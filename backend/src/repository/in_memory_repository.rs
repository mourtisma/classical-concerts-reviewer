use crate::model::base_model::BaseModel;

use super::{base_repository::BaseRepository, error::{RepositoryError, RepositoryErrorType}, list_options::ListOptions};

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

    fn create(&mut self, data: M) -> M {
        self.data.push(data);
        data
    }

    fn update(&mut self, id: &str, data: M) -> Result<M, RepositoryError> {
        let items = &mut self.data;
        if let Some(mut item) = items.iter_mut().find(|x| x.id() == id).cloned() {
            item = data;
            Ok(item)
        } else {
            Err(RepositoryError {
                error_type: RepositoryErrorType::NotFound,
                message: None
            })
        }
    }

    fn delete(&mut self, id: &str) -> Result<(), RepositoryError> {
        let items = &mut self.data;
        let num_items_before = items.len();

        if let Some(_) = items.iter_mut().find(|x| x.id() == id).cloned() {
            items.retain(|x| x.id() == id);
            let num_items_after = items.len();
            if num_items_before - num_items_after <= 0 {
                Err(RepositoryError {
                    error_type: RepositoryErrorType::Unknown,
                    message: None
                })
            } else {
                Ok(())
            }
            
        } else {
            Err(RepositoryError {
                error_type: RepositoryErrorType::NotFound,
                message: None
            })
        }
        
        


    }
}