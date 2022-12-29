use std::sync::Arc;

use crate::db::todo::TodoDocument;
use crate::service::Services;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

use super::Server;

pub struct AppState {
    services: Arc<Services>,
}

#[derive(Clone)]
pub struct ActixServer {
    services: Arc<Services>,
}

impl ActixServer {
    pub fn new(services: Services) -> Self {
        Self {
            services: Arc::new(services),
        }
    }
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

#[async_trait::async_trait]
impl Server for ActixServer {
    async fn run(&self) {
        let services = self.services.clone();
        let server = HttpServer::new(move || {
            App::new()
                .app_data(web::Data::new(AppState {
                    services: services.clone(),
                }))
                .service(get_all_todos)
                .service(add_todo)
        })
        .bind(("127.0.0.1", 8080))
        .unwrap()
        .run();
        server.await.unwrap();
    }
}
