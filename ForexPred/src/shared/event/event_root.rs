use async_trait::async_trait;

#[async_trait]
pub trait Event<'a>: Send + Sync {
    async fn handle(&'a self) -> Result<(), String>;
}