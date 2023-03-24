use rocket::serde::{Serialize};
use serde::Deserialize;

#[derive(Debug, Serialize, Deserialize)]
pub struct PlayerDbo {
    pub _id: String,
    pub name: String,
    pub score: f32,
    pub nombre_de_parties: u32
}