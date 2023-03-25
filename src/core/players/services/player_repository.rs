#[async_trait]
pub trait PlayerRepository<Entity, Response> {
    async fn insert_player(&self, player: Entity) -> Response;
    async fn fetch_many(&self) -> Vec<Entity>;
    async fn fetch_one_by_name(&self, name: String) -> Option<Entity>;
}