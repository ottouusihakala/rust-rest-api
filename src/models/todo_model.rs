use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Todo {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub task: String,
    pub completed: bool
}

impl Todo {
    pub fn new(task: &dyn ToString, completed: bool) -> Self {
        Todo {
            id: None,
            task: task.to_string(),
            completed
        }
    }
}
                
