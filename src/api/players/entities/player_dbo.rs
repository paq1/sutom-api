use bson::oid::ObjectId;
use rocket::serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PlayerDbo {
    pub _id: ObjectId,
    pub name: String,
    pub score: f32,
    pub nombre_de_parties: u32
}