use mongodb::{Collection, Database};
use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct TodoDocument {
    title: String,
    description: String,
    is_completed: bool,
}

impl TodoDocument {
    pub fn new(title: String, description: String, is_completed: bool) -> Self {
        Self {
            title,
            description,
            is_completed,
        }
    }

    pub fn title(&self) -> String {
        self.title.to_string()
    }

    pub fn description(&self) -> String {
        self.description.to_string()
    }

    pub fn is_completed(&self) -> bool {
        self.is_completed
    }
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
