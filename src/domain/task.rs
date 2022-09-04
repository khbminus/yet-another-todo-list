use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct TaskEntry {
    pub id: i32,
    pub content: String,
    pub done: bool
}