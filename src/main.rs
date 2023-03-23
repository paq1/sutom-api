#[macro_use] extern crate rocket;
mod models;
mod api;
use crate::api::routes::read_test::ressources;

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![ressources])
}