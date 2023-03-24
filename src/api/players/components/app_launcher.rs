use rocket::{Build, Rocket};
use crate::api::players::services::player_repository_mongo::PlayerRepositoryMongo;
use crate::api::players::routes::read_test::ressources;
use crate::api::players::routes::player_read_router::get_players;
use crate::api::players::routes::player_write_router::create_command;

pub struct AppLauncher;

impl AppLauncher {
    pub async fn launch_rocket() -> Rocket<Build> {
        let player_repository = PlayerRepositoryMongo::new().await;

        rocket::build()
            .manage(player_repository)
            .mount("/", routes![ressources, get_players, create_command])
    }
}


