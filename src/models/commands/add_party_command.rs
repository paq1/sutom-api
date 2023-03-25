use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct AddPartyCommand {
    pub taille_du_mot: u32,
    pub nombre_essaies: u32,
    pub nombre_essaies_total: u32
}