use rocket::{Build, Rocket};
use crate::api::players::services::player_repository_mongo::PlayerRepositoryMongo;
use crate::api::players::routes::read_test::ressources;

pub struct AppLauncher;

impl AppLauncher {
    pub async fn launch_rocket() -> Rocket<Build> {
        let player_repository = PlayerRepositoryMongo::new().await;

        rocket::build()
            .manage(player_repository)
            .mount("/", routes![ressources])
    }
}


