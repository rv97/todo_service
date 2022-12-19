pub mod actix;

#[async_trait::async_trait]
pub trait Server: Send + Sync {
    async fn run(&self);
}
