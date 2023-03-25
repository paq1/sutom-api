use mongodb::bson::{Bson, Document};
use mongodb::bson::oid::ObjectId;
use rocket::serde::{Deserialize, Serialize};

use crate::api::players::entities::party_dbo::PartyDbo;
use crate::core::players::entities::player::Player;

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct PlayerDbo {
    pub _id: ObjectId,
    pub name: String,
    pub parties: Vec<PartyDbo>
}

impl From<PlayerDbo> for Player {
    fn from(value: PlayerDbo) -> Self {
        Player {
            name: value.name,
            parties: value.parties
                .into_iter()
                .map(|partie| partie.into())
                .collect::<Vec<_>>()
        }
    }
}

impl From<Document> for PlayerDbo {
    fn from(value: Document) -> Self {
        mongodb::bson::from_bson(Bson::Document(value))
            .unwrap()
    }
}