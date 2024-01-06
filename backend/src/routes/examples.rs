use rocket::{fairing::AdHoc, routes, get, serde::json::Json};
use crate::repository::list_options::ListOptions;
use crate::service::base_service::BaseService;
use crate::repository::in_memory_repository::InMemoryRepository;
use crate::repository::base_repository::BaseRepository;

use crate::model::example::Example;


#[get("/")]
async fn list() -> Json<Vec<Example>> {
    let repository = InMemoryRepository::<Example>::new();
    let mut service = BaseService::<Example> {
        repository:  Box::new(repository),
    };

    let examples = service.get_many(ListOptions{order_by: None, page: None, limit: None});

    Json(examples.to_vec())
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Example resource", |rocket| async {
        rocket.mount("/examples", routes![list])
    })
}