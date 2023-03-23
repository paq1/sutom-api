#[macro_use] extern crate rocket;
use rocket::serde::{Serialize, json::Json};

#[derive(Debug, Serialize)]
#[serde(crate = "rocket::serde")]
struct MyResource {
    id: i32,
    name: String,
    description: String,
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/res")]
fn ressources() -> Json<MyResource> {
    Json(
        MyResource {
            id: 12,
            name: String::from("name"),
            description: String::from("description")
        }
    )
}



#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, ressources])
}