use mongodb::bson::{Bson, Document};
use rocket::serde::{Serialize, Deserialize};
use crate::core::players::entities::party::Party;

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct PartyDbo {
    pub taille_du_mot: u32,
    pub nombre_essaies: u32,
    pub nombre_essaies_total: u32
}

impl From<PartyDbo> for Party {
    fn from(value: PartyDbo) -> Self {
        Party {
            taille_du_mot: value.taille_du_mot,
            nombre_essaies: value.nombre_essaies,
            nombre_essaies_total: value.nombre_essaies_total
        }
    }
}

impl From<Document> for PartyDbo {
    fn from(value: Document) -> Self {
        mongodb::bson::from_bson(Bson::Document(value))
            .unwrap()
    }
}