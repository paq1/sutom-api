use mongodb::{
    bson::doc,
    bson::Document,
    Collection,
    results::InsertOneResult
};
use mongodb::error::Error;
use rocket::futures::{TryFutureExt, TryStreamExt};

use crate::api::players::components::mongo_component::ClientMongoComponent;
use crate::api::players::entities::player_dbo::PlayerDbo;
use crate::core::players::entities::player::Player;
use crate::core::players::errors::custom::CustomError;
use crate::core::players::services::player_repository::PlayerRepository;

pub struct PlayerRepositoryMongo {
    pub collection: Collection<Document>,
}

impl ClientMongoComponent for PlayerRepositoryMongo {}

#[async_trait]
impl PlayerRepository<Player, Result<InsertOneResult, CustomError>> for PlayerRepositoryMongo {
    async fn insert_player(&self, player: Player) -> Result<InsertOneResult, CustomError> {
        if !self.exist(&player).await {
            self.insert_player_without_check(&player).await
        } else {
            Err(CustomError::new("le joueur existe déjà en base"))
        }
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

    async fn fetch_one_by_name(&self, name: String) -> Option<Player> {
        self.collection
            .find_one(
                Some(
                    doc! {
                        "name": name.as_str()
                    }
                ),
                None
            )
            .await
            .map(|dbo_doc_opt|{
                dbo_doc_opt
                    .map(|dbo_doc| {
                        let player_dbo: PlayerDbo = dbo_doc.into();
                        player_dbo.into()
                    })
            })
            .unwrap_or(None)
    }
}

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

    async fn exist(&self, player: &Player) -> bool {
        self.fetch_one_by_name(player.name.clone())
            .await
            .is_some()
    }

    async fn insert_player_without_check(&self, player: &Player) -> Result<InsertOneResult, CustomError> {
        let document: Document = player.clone().into();
        self.collection
            .insert_one(document, None)
            .map_err(|_| CustomError::new("erreur lors de l'insertion en base"))
            .await
    }
}
