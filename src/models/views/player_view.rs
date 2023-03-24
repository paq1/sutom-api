use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct PlayerView {
    pub _id: String,
    pub name: String,
    pub score: f32,
    pub nombre_de_parties: u32
}
