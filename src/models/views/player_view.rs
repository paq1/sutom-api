use rocket::serde::{Deserialize, Serialize};
use crate::models::views::party_view::PartyView;

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct PlayerView {
    pub _id: String,
    pub name: String,
    pub last_party_date: Option<String>,
    pub parties: Vec<PartyView>
}
