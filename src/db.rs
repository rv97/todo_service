pub mod todo;

use derive_builder::Builder;
use futures::stream::TryStreamExt;
use mongodb::bson::doc;
use mongodb::options::ClientOptions;
use mongodb::{error::Result as MongoDBResult, Client, Collection, Database};
use thiserror::Error;

use self::todo::{get_todo_collection, TodoDocument};

#[derive(Error, Debug)]
pub enum CollectionsRepositoryError {
    #[error("Cannot get mongodb Connection")]
    MongoDBError(#[from] mongodb::error::Error),
    #[error("Error in accessing Collections {0}")]
    CollectionError(String),
}
#[derive(Builder, Clone)]
pub struct CollectionsRepository {
    pub todo: Collection<TodoDocument>,
}

impl CollectionsRepository {
    pub async fn list_todos(&self) -> Result<Vec<TodoDocument>, CollectionsRepositoryError> {
        let mut cursor = self
            .todo
            .find(None, None)
            .await
            .map_err(|e| CollectionsRepositoryError::MongoDBError(e))?;

        let mut todos_vec: Vec<TodoDocument> = vec![];
        while let Some(td) = cursor.try_next().await? {
            todos_vec.push(td);
        }
        Ok(todos_vec)
    }

    pub async fn add_todo(&self, todo: TodoDocument) -> MongoDBResult<String> {
        let inserted_todo = self.todo.insert_one(todo, None).await?;
        Ok(inserted_todo.inserted_id.to_string())
    }
}
pub async fn get_collections() -> Result<CollectionsRepository, CollectionsRepositoryError> {
    let db_connection = get_connection()
        .await
        .map_err(|e| CollectionsRepositoryError::MongoDBError(e))?;
    let todo_collections = get_todo_collection(db_connection);

    let collections_repository = CollectionsRepositoryBuilder::default()
        .todo(todo_collections)
        .build()
        .map_err(|err| CollectionsRepositoryError::CollectionError(err.to_string()))?;
    Ok(collections_repository)
}

async fn get_connection() -> mongodb::error::Result<Database> {
    let client_options = ClientOptions::parse(
        "mongodb+srv://admin:admin@cluster0.mb32jxr.mongodb.net/?retryWrites=true&w=majority",
    )
    .await?;

    let client = Client::with_options(client_options)?;

    let db = client.database("todoservice");
    Ok(db)
}
