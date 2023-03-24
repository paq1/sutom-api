#[macro_use] extern crate rocket;
mod models;
mod api;
mod core;

use rocket::{Build, Rocket};
use crate::api::players::components::app_launcher::AppLauncher;

#[launch]
async fn rocket() -> Rocket<Build> {
    AppLauncher::launch_rocket().await.unwrap()
}