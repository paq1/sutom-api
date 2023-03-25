use crate::core::players::entities::party::Party;
use crate::core::players::errors::custom::CustomError;

#[async_trait]
pub trait PlayerRepository<Entity, Response> {
    async fn insert_player(&self, player: Entity) -> Response;
    async fn fetch_many(&self) -> Vec<Entity>;
    async fn fetch_one_by_name(&self, name: String) -> Option<Entity>;
    async fn add_party(&self, name: String, party: Party) -> Result<(), CustomError>;
}