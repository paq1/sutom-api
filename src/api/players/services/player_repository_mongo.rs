use std::any::Any;
use bson::{doc, Document};
use mongodb::{Collection, Cursor};
use mongodb::results::InsertOneResult;
use crate::api::players::entities::player_dbo::PlayerDbo;
use crate::core::players::entities::player::Player;
use crate::core::players::services::player_repository::PlayerRepository;
use mongodb::error::Error;

pub struct PlayerRepositoryMongo {
    pub collection: Collection<Document>
}

impl PlayerRepositoryMongo {
    async fn find_all(&self) -> Result<Vec<Player>, Error> {
        // let cursor: Cursor<Document> = self.collection.find(None, None).await?;
        // let mut documents: Vec<Document> = vec![];
        // for result in cursor {
        //     match result {
        //         Ok(document) => documents.push(document),
        //         Err(e) => return e,
        //     }
        // }
        // let docs = documents
        //     .iter()
        //     .map(|doc| {
        //         Player {
        //             name: String::from("player_dbo"),
        //             score: 0f32,
        //             nombre_de_parties: 0
        //         }
        //     })
        //     .collect::<Vec<Player>>();
        // Ok(docs)

        Ok(vec![])
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