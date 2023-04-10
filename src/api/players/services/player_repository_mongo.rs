use chrono::Datelike;
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
use crate::core::players::entities::party::Party;
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

    async fn delete_player(&self, player_name: &str) -> Result<(), CustomError> {
        if self.fetch_one_by_name(player_name.into()).await.is_ok() {
            self.delete_player_without_check(player_name).await
        } else {
            Err(CustomError::new("le joueur n'éxiste pas"))
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

    async fn fetch_one_by_name(&self, name: String) -> Result<Player, CustomError> {
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
                        let player: Player = player_dbo.into();
                        Ok(player)
                    })
                    .unwrap_or(Err(CustomError::new("impossible de recupere le joueur")))
            })
            .unwrap()
            .map_err(|err| CustomError::new(err.message.as_str()))
    }

    async fn add_party(&self, name: String, party: Party) -> Result<(), CustomError> {
        let party_ref = &party.clone();
        self.fetch_one_by_name(name.clone())
            .and_then(|player| async move {
                let player_ref = &player;
                let date_du_jour = chrono::Local::now();
                let date_now_str = &format!(
                    "{}-{}-{}",
                    date_du_jour.day(),
                    date_du_jour.month(),
                    date_du_jour.year()
                );
                let date_derniere_partie_opt = player_ref.last_party_date.clone();

                match date_derniere_partie_opt {
                    Some(date) => {
                        if date != date_now_str.clone() {
                            self.simple_add_party(party_ref.clone(), player_ref)
                                .await
                        } else {
                            Err(CustomError::new("vous avez deja ajouté une partie aujourd'hui"))
                        }
                    },
                    None => {
                        self.simple_add_party(party.clone(), player_ref)
                            .await
                    }
                }
            })
            .await
    }
}

impl PlayerRepositoryMongo {
    async fn simple_add_party(&self, party: Party, old_player: &Player) -> Result<(), CustomError> {
        let filter = doc! { "name": old_player.name.as_str() };
        let updated_player = old_player.add_party(party);
        let date_du_jour = chrono::Local::now();
        let date_now_str = format!(
            "{}-{}-{}",
            date_du_jour.day(),
            date_du_jour.month(),
            date_du_jour.year()
        );
        let update = doc! {
            "$set": {
                "last_party_date": date_now_str,
                "parties": updated_player.parties
            }
        };
        self.collection
            .update_one(filter, update, None)
            .await
            .map(|_| ())
            .map_err(|_| CustomError::new("erreur lors de l'ecriture"))
    }

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
            .is_ok()
    }

    async fn insert_player_without_check(&self, player: &Player) -> Result<InsertOneResult, CustomError> {
        let document: Document = player.clone().into();
        self.collection
            .insert_one(document, None)
            .map_err(|_| CustomError::new("erreur lors de l'insertion en base"))
            .await
    }

    async fn delete_player_without_check(&self, player_name: &str) -> Result<(), CustomError> {
        let document: Document = doc! {"name": player_name};
        self.collection
            .delete_one(document, None)
            .await
            .map(|_| ())
            .map_err(|_| CustomError::new("erreur lors de la suppression"))
    }

}
