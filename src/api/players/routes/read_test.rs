use rocket::serde::{json::Json};
use crate::models::sample_resource::MyResource;

#[get("/")]
pub fn ressources() -> Json<MyResource> {
    Json(
        MyResource :: new (
            12,
            String::from("name"),
            String::from("description")
        )
    )
}
