use std::marker::PhantomData;

use rocket::{fairing::AdHoc, routes, get, serde::json::Json};
use crate::repository::list_options::ListOptions;
use crate::service::base_service::BaseService;
use crate::repository::in_memory_repository::InMemoryRepository;
use crate::repository::base_repository::BaseRepository;

use crate::model::example::Example;
use crate::service::result::SuccessGetManyResult;


#[get("/")]
fn list() -> Json<SuccessGetManyResult<Example>> {
    let repository = InMemoryRepository::<Example>::new();
    let service = BaseService::<Example> {
        repository:  Box::new(repository),
    };
    

    let examples = service.get_many(ListOptions{order_by: None, page: None,limit: None});
    
    Json(examples)
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Example resource", |rocket| async {
        rocket.mount("/examples", routes![list])
    })
}