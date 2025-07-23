use serde::{Serialize, Deserialize};

// Todo model, similar to a Java POJO
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Todo {
    pub id: u32,
    pub title: String,
    pub completed: bool,
}
