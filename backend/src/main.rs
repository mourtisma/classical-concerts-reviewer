use std::env;

use dotenvy::dotenv;
use rocket::launch;
use routes::examples;
use sea_orm::Database;

mod routes;
mod model;
mod dto;
mod transformer;
mod repository;
mod service;
mod status;


#[launch]
async fn rocket() -> _ {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL variable not specified");
    let connection = match Database::connect(database_url).await {
        Ok(connection) => connection,
        Err(e) => panic!("{:?}",e)
    };
    rocket::build().manage(connection).attach(examples::stage())
}