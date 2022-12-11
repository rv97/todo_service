use mongodb::{Collection, Database};
use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct TodoDocument {
    title: String,
    description: String,
    is_completed: bool,
}

impl Default for TodoDocument {
    fn default() -> Self {
        Self {
            is_completed: false,
            title: "".to_string(),
            description: "".to_string(),
        }
    }
}

pub fn get_todo_collection(db_connection: Database) -> Collection<TodoDocument> {
    let collection = db_connection.collection::<TodoDocument>("todos");
    collection
}
