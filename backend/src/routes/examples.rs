use std::marker::PhantomData;

use rocket::http::Status;
use rocket::{fairing::AdHoc, routes, get, post, put, delete, serde::json::Json};
use crate::repository::list_options::ListOptions;
use crate::service::base_service::BaseService;
use crate::repository::in_memory_repository::InMemoryRepository;
use crate::repository::base_repository::BaseRepository;

use crate::model::example::{self, Example};
use crate::service::error::{ErrorResult, ApiError};
use crate::service::result::{SuccessCreateResult, SuccessGetManyResult, SuccessGetOneResult, SuccessUpdateResult};


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
fn detail<'a>(id: &str) -> Result<Json<SuccessGetOneResult<Example<'a>>>, (Status, Json<ErrorResult<'a>>)> {
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
fn create<'a>(example: Json<Example<'a>>) -> Result<(Status, Json<SuccessCreateResult<Example<'a>>>), (Status, Json<ErrorResult<'a>>)> {
    let repository = InMemoryRepository::<Example>::new();
    let mut service = BaseService::<Example> {
        repository:  Box::new(repository),
    };
    
    match service.create(example.0) {
        Ok(example) => Ok((Status::Created, Json(example))),
        Err(api_error) => {
            Err((api_error.http_status(), Json(api_error.to_result())))
        }
    }


}

#[put("/<id>", data="<example>")]
fn update<'a>(id: &'a str, example: Json<Example<'a>>) -> Result<Json<SuccessUpdateResult<Example<'a>>>, (Status, Json<ErrorResult<'a>>)> {
    let repository = InMemoryRepository::<Example>::new();
    let mut service = BaseService::<Example> {
        repository:  Box::new(repository),
    };
    
    match service.update(id, example.0) {
        Ok(res) => Ok(Json(res)),
        Err(api_error) => {
            Err((api_error.http_status(), Json(api_error.to_result())))
        }
    }

}

#[delete("/<id>")]
fn delete<'a>(id: &str) -> Result<Status, (Status, Json<ErrorResult>)> {
    let repository = InMemoryRepository::<Example>::new();
    let mut service = BaseService::<Example> {
        repository:  Box::new(repository),
    };
    
    match service.delete(id) {
        Ok(_) => Ok(Status::NoContent),
        Err(api_error) => {
            Err((api_error.http_status(), Json(api_error.to_result())))
        }
    }

}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Example resource", |rocket| async {
        rocket.mount("/examples", routes![list, detail, create, update, delete])
    })
}