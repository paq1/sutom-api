use mongodb::{
    Collection,
    Cursor,
    bson::Bson,
    bson::doc,
    bson::Document
};
use mongodb::results::InsertOneResult;
use crate::core::players::entities::player::Player;
use crate::core::players::services::player_repository::PlayerRepository;
use mongodb::error::Error;
use rocket::futures::{TryStreamExt};
use crate::api::players::components::mongo_component::ClientMongoComponent;
use crate::api::players::entities::player_dbo::PlayerDbo;

pub struct PlayerRepositoryMongo {
    pub collection: Collection<Document>
}
impl ClientMongoComponent for PlayerRepositoryMongo {}

impl PlayerRepositoryMongo {
    async fn find_all(&self) -> Result<Vec<Player>, Error> {
        let cursor: Cursor<Document> = self.collection.find(None, None).await?;

        let bsons = cursor
            .try_collect::<Vec<Document>>()
            .await?;

        let players = bsons
            .into_iter()
            .map(|doc| {
                    let player: PlayerDbo = mongodb::bson::from_bson(Bson::Document(doc)).unwrap();
                    Some(player)
                }
            )
            .map(|dbo_opt| {
                let dbo = dbo_opt.unwrap();
                Player {
                    name: dbo.name.clone(),
                    score: dbo.score,
                    nombre_de_parties: dbo.nombre_de_parties
                }
            })
            .collect::<Vec<Player>>();

        Ok(players)
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