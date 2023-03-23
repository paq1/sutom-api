#[async_trait]
pub trait PlayerRepository<Entity, Response> {
    async fn insert_player(&self, player: Entity) -> Response ;
    async fn fetch_many(&self) -> Vec<Entity>;
}