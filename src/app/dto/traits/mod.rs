use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait SessionInterface {
    async fn start(&self) -> (Uuid, String);
    async fn end(&self);
}
