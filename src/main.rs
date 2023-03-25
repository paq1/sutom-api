#[macro_use] extern crate rocket;

use rocket::{Build, Rocket};

use crate::api::players::components::app_launcher::AppLauncher;

mod models;
mod api;
mod core;

#[launch]
async fn rocket() -> Rocket<Build> {
    dotenv::dotenv().ok();
    AppLauncher::launch_rocket().await.unwrap()
}