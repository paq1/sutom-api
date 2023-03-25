use mongodb::bson;
use mongodb::bson::{Bson, doc};

use crate::models::views::party_view::PartyView;

#[derive(Clone)]
pub struct Party {
    pub taille_du_mot: u32,
    pub nombre_essaies: u32,
    pub nombre_essaies_total: u32
}

impl From<Party> for PartyView {
    fn from(value: Party) -> Self {
        PartyView {
            taille_du_mot: value.taille_du_mot,
            nombre_essaies: value.nombre_essaies,
            nombre_essaies_total: value.nombre_essaies_total
        }
    }
}

impl From<Party> for Bson {
    fn from(value: Party) -> Self {
        let document = doc! {
            "taille_du_mot": value.taille_du_mot,
            "nombre_essaies": value.nombre_essaies,
            "nombre_essaies_total": value.nombre_essaies_total
        };
        bson::to_bson(&document).unwrap()
    }
}