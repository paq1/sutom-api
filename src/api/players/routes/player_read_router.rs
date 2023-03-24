use rocket::serde::{json::Json};
use rocket::State;
use crate::api::players::services::player_repository_mongo::PlayerRepositoryMongo;
use crate::core::players::services::player_repository::PlayerRepository;
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
            .map(|player| {
                PlayerView {
                    _id: "unknown".to_string(),
                    name: player.name.clone(),
                    score: player.score,
                    nombre_de_parties: player.nombre_de_parties
                }
            })
            .collect::<Vec<PlayerView>>()
    )
}
