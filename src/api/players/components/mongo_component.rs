use mongodb::{options::ClientOptions, Client, Collection, bson::Document};
use mongodb::error::Error;

#[async_trait]
pub trait ClientMongoComponent {
    const URI: &'static str = "mongodb://localhost:27017";
    async fn connection() -> Result<Client, Error> {
        let mut client_option = ClientOptions::parse(Self::URI).await?;
        client_option.app_name = Some(String::from("Sutom App"));
        Client::with_options(client_option)
    }

    async fn collection_player() -> Result<Collection<Document>, mongodb::error::Error> {
        Ok(
            Self::connection()
                .await?
                .database("sutom")
                .collection("player")
        )
    }
}
