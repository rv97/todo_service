use derive_builder::Builder;

use crate::db::{todo::TodoDocument, CollectionsRepository};

#[derive(Clone, Builder)]
pub struct TodoService {
    collections: CollectionsRepository,
}

impl TodoService {
    pub async fn list_todos(&self) -> Result<Vec<TodoDocument>, Box<dyn std::error::Error>> {
        let todos = self.collections.list_todos().await?;
        Ok(todos)
    }
}
