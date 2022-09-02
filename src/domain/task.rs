use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct TaskEntry {
    pub id: u32,
    pub content: String,
    pub done: bool
}