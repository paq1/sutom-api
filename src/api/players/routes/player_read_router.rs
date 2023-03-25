use rocket::http::Status;
use rocket::response::status;
use rocket::serde::{json::Json};
use rocket::State;
use crate::api::players::services::player_repository_mongo::PlayerRepositoryMongo;
use crate::core::players::services::player_repository::PlayerRepository;
use crate::models::views::json_data_response::JsonDataResponse;
use crate::models::views::player_view::PlayerView;

#[get("/players")]
pub async fn get_players(
    player_repository: &State<PlayerRepositoryMongo>
) -> Json<Vec<PlayerView>> {
    Json(
        player_repository
            .fetch_many()
            .await
            .into_iter()
            .map(|player| player.into())
            .collect::<Vec<_>>()
    )
}

#[get("/players/<name>")]
pub async fn get_player_by_name(
    name: &str,
    player_repository: &State<PlayerRepositoryMongo>
) -> Result<Json<PlayerView>, status::Custom<Json<JsonDataResponse>>> {
    player_repository
        .fetch_one_by_name(name.into())
        .await
        .map(|player| Ok(Json(player.into())))
        .unwrap_or(
            Err(
                status::Custom(
                    Status::NotFound,
                    Json(
                        JsonDataResponse {
                            message: format!("le joueur {} n'existe pas", name)
                        }
                    )
                )
            )
        )
}
