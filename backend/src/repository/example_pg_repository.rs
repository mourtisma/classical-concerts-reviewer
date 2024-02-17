use std::{marker::PhantomData, vec};

use rocket_db_pools::{diesel::prelude::*, Connection};
use crate::{db::Ccr, model::example::Example, schema::examples::dsl::*};

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

      

}