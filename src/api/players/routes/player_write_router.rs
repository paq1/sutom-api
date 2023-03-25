use rocket::http::Status;
use rocket::response::status;
use rocket::serde::json::Json;
use rocket::State;

use crate::api::players::services::player_repository_mongo::PlayerRepositoryMongo;
use crate::core::players::entities::player::Player;
use crate::core::players::services::player_repository::PlayerRepository;
use crate::models::commands::create_player_command::CreatePlayerCommand;
use crate::models::views::json_data_response::JsonDataResponse;

#[post("/players/commands/create", data="<create_command>")]
pub async fn create_command(
    player_repository: &State<PlayerRepositoryMongo>,
    create_command: Json<CreatePlayerCommand>
) -> Result<Json<JsonDataResponse>, status::Custom<Json<JsonDataResponse>>> {

    let cmd = create_command.0;

    player_repository
        .insert_player(
            Player {
                name: cmd.name.clone(),
                score: cmd.score,
                nombre_de_parties: cmd.nombre_de_parties
            }
        )
        .await
        .map(|_| {
            Json(
                JsonDataResponse {
                    message: String::from("inserted")
                }
            )
        })
        .map_err(|err| {
            status::Custom(
                Status::BadRequest,
                Json(
                    JsonDataResponse {
                        message: err.message
                    }
                )
            )
        })
}
