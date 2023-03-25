use mongodb::bson::{doc, Document};

use crate::models::views::player_view::PlayerView;

#[derive(Clone)]
pub struct Player {
    pub name: String,
    pub score: f32,
    pub nombre_de_parties: u32
}

impl From<Player> for PlayerView {
    fn from(value: Player) -> Self {
        PlayerView {
            _id: "unknown".to_string(),
            name: value.name,
            score: value.score,
            nombre_de_parties: value.nombre_de_parties
        }
    }
}

impl From<Player> for Document {
    fn from(value: Player) -> Self {
        doc! {
            "name": value.name,
            "score": value.score,
            "nombre_de_parties": value.nombre_de_parties
        }
    }
}
