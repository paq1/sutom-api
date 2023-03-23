use std::any::Any;
use mongodb::{bson::doc, options::ClientOptions, Client, Collection};
use mongodb::error::Error;
use crate::api::players::entities::player_dbo::PlayerDbo;
use crate::api::players::services::player_repository_mongo::PlayerRepositoryMongo;
use crate::core::players::services::player_repository::PlayerRepository;
use crate::core::players::entities::player::Player;

#[async_trait]
pub trait ClientMongoComponent {
    const uri: String = "mongodb://localhost:27017".to_string();
    async fn connection() -> Result<Client, Error> {
        let mut client_option = ClientOptions::parse(Self::uri).await?;
        client_option.app_name = Some(String::from("Sutom App"));
        Client::with_options(client_option)
    }

    async fn collection_player(client: Client) -> Collection<PlayerDbo> {
        client
            .database("sutom")
            .collection("player")
    }

    const repo_player: PlayerRepositoryMongo = PlayerRepositoryMongo {
        collection: Self::collection_player(Self::connection())
    };

}
