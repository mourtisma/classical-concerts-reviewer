use std::marker::PhantomData;

use rocket::http::Status;
use rocket::State;
use rocket::{fairing::AdHoc, routes, get, post, put, delete, serde::json::Json};
use sea_orm::DatabaseConnection;

use crate::dto::example_dto::{ExampleCreateDto, ExampleGetDto, ExampleUpdateDto};
use crate::dto::example_with_relation_dto::{ExampleWithRelationCreateDto, ExampleWithRelationGetDto, ExampleWithRelationUpdateDto};
use crate::model::prelude::{ExampleActiveModel, ExampleSeaOrm};
use crate::repository::base_seaorm_repository::BaseSeaOrmRepository;
use crate::repository::example_with_relation_repository::ExampleWithRelationRepository;
use crate::repository::list_options::ListOptions;

use crate::service::error::{ErrorResult, ApiError};
use crate::service::base_service::BaseService;
use crate::service::example_with_relation_service::ExampleWithRelationService;
use crate::service::result::{SuccessCreateResult, SuccessGetManyResult, SuccessGetOneResult, SuccessUpdateResult};
use crate::transformer::example_transformer::ExampleTransformer;

type ExampleService<'a> = BaseService<'a, ExampleSeaOrm, ExampleGetDto, ExampleCreateDto, ExampleUpdateDto, ExampleTransformer, ExampleActiveModel>;

fn get_service<'a>(connection: &'a State<DatabaseConnection>) -> ExampleWithRelationService<'a> {
    let repository = ExampleWithRelationRepository {
        connection,
    };
    
    ExampleWithRelationService {
        repository,
    }
}

#[get("/")]
async fn list<'a>(connection: &'a State<DatabaseConnection>) -> Result<Json<SuccessGetManyResult<ExampleWithRelationGetDto>>, (Status, Json<ErrorResult<'a>>)> {
    let mut service = get_service(connection);
    

    let examples_result = service.get_many(ListOptions{order_by: None, page: None,limit: None}).await;
    
    match examples_result {
        Ok(examples) => Ok(Json(examples)),
        Err(api_error) => {
            Err((api_error.http_status(), Json(api_error.to_result())))
        }
    }
    
}

/* #[get("/<id>")]
async fn detail<'a>(connection: &'a State<DatabaseConnection>, id: &'a str) -> Result<Json<SuccessGetOneResult<ExampleGetDto>>, (Status, Json<ErrorResult<'a>>)> {
    let mut service = get_example_service(connection);
    
    match service.get_one(id).await {
        Ok(res) => Ok(Json(res)),
        Err(api_error) => {
            Err((api_error.http_status(), Json(api_error.to_result())))
        }
    }

} */

#[post("/", data="<example>")]
async fn create<'a>(connection: &'a State<DatabaseConnection>, example: Json<ExampleWithRelationCreateDto>) -> Result<(Status, Json<SuccessCreateResult<ExampleWithRelationGetDto>>), (Status, Json<ErrorResult<'a>>)> {
    let mut service = get_service(connection);
    
    match service.create(example.0).await {
        Ok(example) => Ok((Status::Created, Json(example))),
        Err(api_error) => {
            Err((api_error.http_status(), Json(api_error.to_result())))
        }
    }


}

#[put("/<id>", data="<example>")]
async fn update<'a>(connection: &'a State<DatabaseConnection>, id: &'a str, example: Json<ExampleWithRelationUpdateDto>) -> Result<Json<SuccessUpdateResult<ExampleWithRelationGetDto>>, (Status, Json<ErrorResult<'a>>)> {
    let mut service = get_service(connection);
    
    match service.update(id, example.0).await {
        Ok(res) => Ok(Json(res)),
        Err(api_error) => {
            Err((api_error.http_status(), Json(api_error.to_result())))
        }
    }

}

/*#[delete("/<id>")]
async fn delete<'a>(connection: &'a State<DatabaseConnection>, id: &'a str) -> Result<Status, (Status, Json<ErrorResult<'a>>)> {
    let mut service = get_example_service(connection);
    
    match service.delete(id).await {
        Ok(_) => Ok(Status::NoContent),
        Err(api_error) => {
            Err((api_error.http_status(), Json(api_error.to_result())))
        }
    }

} */

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Example resource with relations", |rocket| async {
        rocket.mount("/examples-with-relations", routes![list, create, update])
    })
}