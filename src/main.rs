use todoservice::server::{actix::ActixServer, Server};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let collections = todoservice::db::get_collections().await?;
    let services = todoservice::service::get_services(collections)?;
    let server: Box<dyn Server> = Box::new(ActixServer::new(services));
    server.run().await;
    Ok(())
}
