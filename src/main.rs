#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let collections = todoservice::db::get_collections().await?;
    let services = todoservice::service::get_services(collections)?;
    println!("{:#?}", services.todo.list_todos().await?);
    Ok(())
}
