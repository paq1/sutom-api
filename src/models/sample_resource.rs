use rocket::serde::{Serialize};

#[derive(Debug, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct MyResource {
    id: i32,
    name: String,
    description: String,
}

impl MyResource {
    pub fn new(
        id: i32,
        name: String,
        description: String
    ) -> Self {
        MyResource {
            id, name, description
        }
    }
}