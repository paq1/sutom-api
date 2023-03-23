use rocket::serde::{Serialize};

#[derive(Debug, Serialize)]
pub struct PlayerDbo {
    pub _id: String,
    pub name: String,
    pub score: f32,
    pub nombre_de_parties: u32
}