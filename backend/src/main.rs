use rocket::launch;
use routes::examples;

mod routes;
mod model;
mod repository;
mod service;

#[launch]
fn rocket() -> _ {
    rocket::build().attach(examples::stage())
}