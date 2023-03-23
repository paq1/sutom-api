#[macro_use] extern crate rocket;
mod models;
mod api;
mod core;

use crate::api::players::components::main_component::Component;
use crate::api::players::routes::read_test::ressources;

#[launch]
fn rocket() -> _ {
    let component = Component {};

    rocket::build().mount("/", routes![ressources])
}