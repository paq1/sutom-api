use mongodb::bson::{Bson, Document};
use mongodb::bson::oid::ObjectId;
use rocket::serde::{Serialize, Deserialize};
use crate::core::players::entities::player::Player;

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct PlayerDbo {
    pub _id: ObjectId,
    pub name: String,
    pub score: f32,
    pub nombre_de_parties: u32
}

impl From<PlayerDbo> for Player {
    fn from(value: PlayerDbo) -> Self {
        Player {
            name: value.name,
            score: value.score,
            nombre_de_parties: value.nombre_de_parties
        }
    }
}

impl From<Document> for PlayerDbo {
    fn from(value: Document) -> Self {
        mongodb::bson::from_bson(Bson::Document(value)).unwrap()
    }
}