use derive_builder::Builder;

use crate::db::CollectionsRepository;

use self::todo::{TodoService, TodoServiceBuilder};

pub mod todo;
#[derive(Builder)]
pub struct Services {
    pub todo: TodoService,
}

pub fn get_services(
    collections: CollectionsRepository,
) -> Result<Services, Box<dyn std::error::Error>> {
    let todo_service = TodoServiceBuilder::default()
        .collections(collections)
        .build()?;
    let services = ServicesBuilder::default().todo(todo_service).build()?;
    Ok(services)
}
