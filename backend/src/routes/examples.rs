use std::marker::PhantomData;

use rocket::http::Status;
use rocket::{fairing::AdHoc, routes, get, post, put, delete, serde::json::Json};
use rocket_db_pools::{Connection, Database};
use crate::db::Ccr;
use crate::repository::example_pg_repository::ExamplePgRepository;
use crate::repository::list_options::ListOptions;


use crate::model::example::{self, Example, ExampleSave};
use crate::service::error::{ErrorResult, ApiError};
use crate::service::example_service::ExampleService;
use crate::service::result::{SuccessCreateResult, SuccessGetManyResult, SuccessGetOneResult, SuccessUpdateResult};


#[get("/")]
async fn list<'a>(connection: Connection<Ccr>) -> Result<Json<SuccessGetManyResult<Example>>, (Status, Json<ErrorResult<'a>>)> {
    let repository = ExamplePgRepository {
        connection,
        _phantomData: PhantomData
    };
    let mut service = ExampleService {
        repository,
    };
    

    let examples_result = service.get_many(ListOptions{order_by: None, page: None,limit: None}).await;
    
    match examples_result {
        Ok(examples) => Ok(Json(examples)),
        Err(api_error) => {
            Err((api_error.http_status(), Json(api_error.to_result())))
        }
    }
    
}

#[get("/<id>")]
async fn detail<'a>(connection: Connection<Ccr>, id: &str) -> Result<Json<SuccessGetOneResult<Example>>, (Status, Json<ErrorResult<'a>>)> {
    let repository = ExamplePgRepository {
        connection,
        _phantomData: PhantomData
    };
    let mut service = ExampleService {
        repository,
    };
    
    match service.get_one(id).await {
        Ok(res) => Ok(Json(res)),
        Err(api_error) => {
            Err((api_error.http_status(), Json(api_error.to_result())))
        }
    }

}

#[post("/", data="<example>")]
async fn create<'a>(connection: Connection<Ccr>, example: Json<ExampleSave>) -> Result<(Status, Json<SuccessCreateResult<Example>>), (Status, Json<ErrorResult<'a>>)> {
    let repository = ExamplePgRepository {
        connection,
        _phantomData: PhantomData
    };
    let mut service = ExampleService {
        repository,
    };
    
    match service.create(example.0).await {
        Ok(example) => Ok((Status::Created, Json(example))),
        Err(api_error) => {
            Err((api_error.http_status(), Json(api_error.to_result())))
        }
    }


}

#[put("/<id>", data="<example>")]
async fn update<'a>(connection: Connection<Ccr>, id: &'a str, example: Json<ExampleSave>) -> Result<Json<SuccessUpdateResult<Example>>, (Status, Json<ErrorResult<'a>>)> {
    let repository = ExamplePgRepository {
        connection,
        _phantomData: PhantomData
    };
    let mut service = ExampleService {
        repository,
    };
    
    match service.update(id, example.0).await {
        Ok(res) => Ok(Json(res)),
        Err(api_error) => {
            Err((api_error.http_status(), Json(api_error.to_result())))
        }
    }

}

/*#[delete("/<id>")]
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

} */

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Example resource", |rocket| async {
        rocket.attach(Ccr::init())
              .mount("/examples", routes![list, detail, create, update])
    })
}