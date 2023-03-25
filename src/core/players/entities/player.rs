use mongodb::bson::{doc, Document};

use crate::core::players::entities::party::Party;
use crate::models::views::player_view::PlayerView;

#[derive(Clone)]
pub struct Player {
    pub name: String,
    pub parties: Vec<Party>
}

impl From<Player> for PlayerView {
    fn from(value: Player) -> Self {
        PlayerView {
            _id: "unknown".to_string(),
            name: value.name,
            parties: value
                .parties
                .into_iter()
                .map(|party| party.into())
                .collect::<Vec<_>>()
        }
    }
}

impl From<Player> for Document {
    fn from(value: Player) -> Self {
        doc! {
            "name": value.name,
            "parties": value.parties
        }
    }
}
