#[macro_use] extern crate rocket;
mod models;
mod api;
mod core;

use crate::api::players::components::app_launcher::AppLauncher;

#[launch]
async fn rocket() -> _ {
    AppLauncher::launch_rocket().await
}