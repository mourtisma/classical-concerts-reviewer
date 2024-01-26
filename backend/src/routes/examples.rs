use rocket::http::Status;
use rocket::{fairing::AdHoc, routes, get, post, serde::json::Json};
use crate::repository::list_options::ListOptions;
use crate::service::base_service::BaseService;
use crate::repository::in_memory_repository::InMemoryRepository;
use crate::repository::base_repository::BaseRepository;

use crate::model::example::Example;
use crate::service::error::{ErrorResult, ApiError};
use crate::service::result::{SuccessCreateResult, SuccessGetManyResult, SuccessGetOneResult};


#[get("/")]
fn list<'a>() -> Json<SuccessGetManyResult<Example<'a>>> {
    let repository = InMemoryRepository::<Example>::new();
    let service = BaseService::<Example> {
        repository:  Box::new(repository),
    };
    

    let examples = service.get_many(ListOptions{order_by: None, page: None,limit: None});
    
    Json(examples)
}

#[get("/<id>")]
fn detail<'a>(id: &str) -> Result<Json<SuccessGetOneResult<Example<'a>>>, (Status, Json<ErrorResult>)> {
    let repository = InMemoryRepository::<Example>::new();
    let service = BaseService::<Example> {
        repository:  Box::new(repository),
    };
    
    match service.get_one(id) {
        Ok(res) => Ok(Json(res)),
        Err(api_error) => {
            Err((api_error.http_status(), Json(api_error.to_result())))
        }
    }

}

#[post("/", data="<example>")]
fn create<'a>(example: Json<Example<'a>>) -> Json<SuccessCreateResult<Example<'a>>> {
    let repository = InMemoryRepository::<Example>::new();
    let mut service = BaseService::<Example> {
        repository:  Box::new(repository),
    };
    
    let example = service.create(example.0);
    
    Json(example)

}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Example resource", |rocket| async {
        rocket.mount("/examples", routes![list, detail, create])
    })
}