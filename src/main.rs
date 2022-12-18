use std::{sync::Arc, vec};

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, Result as ActixResult};
use todoservice::{db::todo::TodoDocument, service::Services};

pub struct AppState {
    services: Arc<Services>,
}

#[post("/addtodo")]
async fn add_todo(data: web::Data<AppState>, item: web::Json<TodoDocument>) -> impl Responder {
    let local_todo = TodoDocument::new(item.title(), item.description(), item.is_completed());
    let inserted_todo = data.services.todo.add_todo(local_todo).await.unwrap();
    HttpResponse::Ok().json(web::Json(inserted_todo))
}

#[get("/todos")]
async fn get_all_todos(data: web::Data<AppState>) -> impl Responder {
    let todos = data.services.todo.list_todos().await.unwrap();
    HttpResponse::Ok().json(web::Json(todos))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let collections = todoservice::db::get_collections().await?;
    let services = todoservice::service::get_services(collections)?;
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState {
                services: Arc::new(services.clone()),
            }))
            .service(get_all_todos)
            .service(add_todo)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
    .map_err(|e| e.into())
}
