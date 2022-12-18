use derive_builder::Builder;

use crate::db::{todo::TodoDocument, CollectionsRepository};
use mongodb::error::Result as MongoDBResult;
#[derive(Clone, Builder)]
pub struct TodoService {
    collections: CollectionsRepository,
}

impl TodoService {
    pub async fn list_todos(&self) -> Result<Vec<TodoDocument>, Box<dyn std::error::Error>> {
        let todos = self.collections.list_todos().await?;
        Ok(todos)
    }

    pub async fn add_todo(&self, todo: TodoDocument) -> MongoDBResult<String> {
        let inserted_todo = self.collections.add_todo(todo).await?;
        Ok(inserted_todo)
    }
}
