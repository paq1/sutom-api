use crate::core::players::entities::party::Party;
use crate::core::players::errors::custom::CustomError;

#[async_trait]
pub trait PlayerRepository<Entity, Response> {
    async fn insert_player(&self, player: Entity) -> Response;
    async fn delete_player(&self, player_name: &str) -> Result<(), CustomError>;
    async fn fetch_many(&self) -> Vec<Entity>;
    async fn fetch_one_by_name(&self, name: String) -> Result<Entity, CustomError>;
    async fn add_party(&self, name: String, party: Party) -> Result<(), CustomError>;
}