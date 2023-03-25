use mongodb::bson::{doc, Document};

use crate::core::players::entities::party::Party;
use crate::models::views::player_view::PlayerView;

#[derive(Clone)]
pub struct Player {
    pub name: String,
    pub parties: Vec<Party>
}

impl Player {
    pub fn add_party(&self, party: Party) -> Self {
        Player {
            name: self.name.clone(),
            parties: self.parties.clone().into_iter().chain(vec![party]).collect::<Vec<_>>()
        }
    }
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
