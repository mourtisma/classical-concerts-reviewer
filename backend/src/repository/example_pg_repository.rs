use std::{marker::PhantomData, vec};

use rocket_db_pools::{diesel::prelude::*, Connection};
use crate::{db::Ccr, model::example::Example, schema::examples};

use super::{error::{RepositoryError, RepositoryErrorType}, list_options::ListOptions};

pub struct ExamplePgRepository<'a> {
    pub connection: Connection<Ccr>,
    pub _phantomData: PhantomData<&'a String>
}


impl<'a> ExamplePgRepository<'a> {

    pub async fn get_many(&mut self, options: ListOptions) -> Result<Vec<Example>, RepositoryError<'a>> {
        let get_many_result = examples::table.load::<Example>(&mut self.connection).await;
        
        if get_many_result.is_err() {
            Err(RepositoryError {
                error_type: RepositoryErrorType::Unknown,
                message: Some("An unknow error occurred"),
                diesel_error: get_many_result.err()
            })
        } else {
            match get_many_result.ok() {
                Some(examples) => Ok(examples),
                None => Ok(vec![]),    
            }
        }
        }

      

}