use mongodb::{
    Collection,
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

    pub async fn new() -> Result<Self, mongodb::error::Error> {
        Ok(
            Self {
                collection: {
                    Self::collection_player().await?
                }
            }
        )
    }
}

#[async_trait]
impl PlayerRepository<Player, Result<InsertOneResult, Error>> for PlayerRepositoryMongo {
    async fn insert_player(&self, player: Player) -> Result<InsertOneResult, Error> {
        let document: Document = player.into();
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