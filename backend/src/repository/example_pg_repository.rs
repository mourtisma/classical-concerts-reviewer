use std::{marker::PhantomData, vec};

use rocket_db_pools::{diesel::prelude::*, Connection};
use crate::{db::Ccr, model::example::{Example, ExampleSave}, schema::examples::dsl::*};

use super::{error::{RepositoryError, RepositoryErrorType}, list_options::ListOptions};

pub struct ExamplePgRepository<'a> {
    pub connection: Connection<Ccr>,
    pub _phantomData: PhantomData<&'a String>
}


impl<'a> ExamplePgRepository<'a> {

    pub async fn get_many(&mut self, options: ListOptions) -> Result<Vec<Example>, RepositoryError<'a>> {
        let get_many_result = examples.load::<Example>(&mut self.connection).await;
        
        if get_many_result.is_err() {
            Err(RepositoryError {
                error_type: RepositoryErrorType::Unknown,
                message: Some("An unknow error occurred"),
                diesel_error: get_many_result.err()
            })
        } else {
            match get_many_result.ok() {
                Some(example_list) => Ok(example_list),
                None => Ok(vec![]),    
            }
        }
    }

    pub async fn get_one(&mut self, example_id: &str) -> Result<Option<Example>, RepositoryError<'a>> {
        let get_one_result = examples.filter(id.eq(example_id)).first::<Example>(&mut self.connection).await;
        
        if get_one_result.is_err() {
            let fetch_error = get_one_result.err();
            
            if let Some(diesel_error) = fetch_error {
                match diesel_error {
                    diesel::result::Error::NotFound => Err(RepositoryError {
                        error_type: RepositoryErrorType::NotFound,
                        message: None,
                        diesel_error: Some(diesel_error)
                    }),
                    _ => Err(RepositoryError {
                        error_type: RepositoryErrorType::Unknown,
                        message: Some("An unknow error occurred"),
                        diesel_error: Some(diesel_error)
                    })
                }
            } else {
                Err(RepositoryError {
                    error_type: RepositoryErrorType::Unknown,
                    message: Some("An unknow error occurred"),
                    diesel_error: fetch_error
                })
            }
            
        } else {
            match get_one_result.ok() {
                Some(example) => Ok(Some(example)),
                None => Ok(None),    
            }
        }
    }

    pub async fn create(&mut self, data: ExampleSave) -> Result<Example, RepositoryError<'a>> {
        let insert_result = diesel::insert_into(examples)
        .values(data)
        .returning(Example::as_returning())
        .get_result(&mut self.connection).await;
        
        match insert_result {
            Err(diesel_error) => Err(RepositoryError {
                error_type: RepositoryErrorType::Unknown,
                message: Some("An unknow error occurred"),
                diesel_error: Some(diesel_error)
            }),
            Ok(new_example) => Ok(new_example)
        }
    }

    pub async fn update(&mut self, example_id: &str, data: ExampleSave) -> Result<Example, RepositoryError<'a>> {
        let update_result = diesel::update(examples)
        .filter(id.eq(example_id))
        .set(data)
        .returning(Example::as_returning())
        .get_result(&mut self.connection).await;
        
        if let Err(update_error) = update_result {
                match update_error {
                    diesel::result::Error::NotFound => Err(RepositoryError {
                        error_type: RepositoryErrorType::NotFound,
                        message: None,
                        diesel_error: Some(update_error)
                    }),
                    _ => Err(RepositoryError {
                        error_type: RepositoryErrorType::Unknown,
                        message: Some("An unknow error occurred"),
                        diesel_error: Some(update_error)
                    })
                }
        } else if let Ok(updated_example) = update_result {
            Ok(updated_example) 
        } else {
            Err(RepositoryError {
                error_type: RepositoryErrorType::Unknown,
                message: Some("An unknow error occurred"),
                diesel_error: None
            })
        }
    }

    pub async fn delete(&mut self, example_id: &str) -> Result<(), RepositoryError<'a>> {
        let delete_result = diesel::delete(examples.filter(id.eq(example_id)))
        .execute(&mut self.connection).await;

        if let Ok(number_of_deleted_records) = delete_result {
            if number_of_deleted_records > 0 {
                Ok(())
            } else {
                Err(RepositoryError {
                    error_type: RepositoryErrorType::NotFound,
                        message: None,
                        diesel_error: None
                })
            }
        } else {
            Err(RepositoryError {
                error_type: RepositoryErrorType::Unknown,
                message: Some("An unknow error occurred"),
                diesel_error: None
            })
        }
        
    }

      

}