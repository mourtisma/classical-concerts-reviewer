use std::{marker::PhantomData, vec};

use rocket_db_pools::{diesel::prelude::*, Connection};
use crate::{db::Ccr, model::example::Example, schema::examples};

use super::{list_options::ListOptions};

pub struct ExamplePgRepository<'a> {
    pub connection: &'a mut Connection<Ccr>
}


impl<'a> ExamplePgRepository<'a> {

    pub async fn get_many(&mut self, options: ListOptions) -> Vec<Example> {
        let results = examples::table.load::<Example>(&mut self.connection).await.ok();
        match results {
            Some(examples) => examples,
            None => vec![],
            
        }
    }

}