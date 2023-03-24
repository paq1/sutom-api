use mongodb::{
    Collection,
    bson::doc,
    bson::Document,
};
use mongodb::results::InsertOneResult;
use crate::core::players::entities::player::Player;
use crate::core::players::services::player_repository::PlayerRepository;
use mongodb::error::Error;
use rocket::futures::{TryStreamExt};
use crate::api::players::components::mongo_component::ClientMongoComponent;
use crate::api::players::entities::player_dbo::PlayerDbo;

pub struct PlayerRepositoryMongo {
    pub collection: Collection<Document>,
}

impl ClientMongoComponent for PlayerRepositoryMongo {}

impl PlayerRepositoryMongo {
    async fn find_all(&self) -> Result<Vec<Player>, Error> {
        Ok(
            self.collection
                .find(None, None)
                .await?
                .try_collect::<Vec<Document>>()
                .await?
                .into_iter()
                .map(|doc| doc.into())
                .map(|dbo: PlayerDbo| dbo.into())
                .collect::<Vec<Player>>()
                .into()
        )
    }

    pub async fn new() -> Self {
        let client_opt = Self::connection()
            .await
            .map(|client| Some(client))
            .unwrap_or(None);

        let collection_opt = client_opt
            .map(|client| Self::collection_player(client))
            .or(None);

        let collection = collection_opt.unwrap();

        Self {
            collection
        }
    }
}

#[async_trait]
impl PlayerRepository<Player, Result<InsertOneResult, Error>> for PlayerRepositoryMongo {
    async fn insert_player(&self, player: Player) -> Result<InsertOneResult, Error> {
        let document = doc! {
            "name": player.name,
            "score": player.score,
            "nombre_de_parties": player.nombre_de_parties
        };
        self.collection
            .insert_one(document, None)
            .await
    }

    async fn fetch_many(&self) -> Vec<Player> {
        match self.find_all().await {
            Ok(player) => player,
            _ => {
                eprintln!("une erreur est survenue lors de la lecture");
                vec![]
            }
        }
    }
}