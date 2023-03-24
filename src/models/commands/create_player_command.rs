use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct CreatePlayerCommand {
    pub name: String,
    pub score: f32,
    pub nombre_de_parties: u32
}
