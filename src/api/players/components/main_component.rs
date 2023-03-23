use crate::api::players::components::mongo_component::ClientMongoComponent;
use mongodb::{bson::doc, options::ClientOptions, Client};
use rocket::futures::TryFutureExt;

pub trait MainComponent: ClientMongoComponent {}

pub struct Component;


// impl ClientMongoComponent for Component {}
// impl MainComponent for Component {}
