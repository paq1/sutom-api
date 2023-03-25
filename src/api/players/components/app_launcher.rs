use rocket::{Build, Rocket};
use crate::api::players::services::player_repository_mongo::PlayerRepositoryMongo;
use crate::api::players::routes::read_test::ressources;
use crate::api::players::routes::player_read_router::get_players;
use crate::api::players::routes::player_read_router::get_player_by_name;
use crate::api::players::routes::player_write_router::create_command;
use crate::core::players::errors::custom::CustomError;

pub struct AppLauncher;

impl AppLauncher {
    pub async fn launch_rocket() -> Result<Rocket<Build>, CustomError> {
        PlayerRepositoryMongo::new().await
            .map(|player_repository| {
                rocket::build()
                    .manage(player_repository)
                    .mount(
                        "/",
                        routes![
                            ressources,
                            get_players,
                            get_player_by_name,
                            create_command
                        ]
                    )
            })
            .map_err(|err| CustomError::new(err.to_string().as_str()))
    }
}


