use serde::{Serialize, Deserialize};
use uuid::Uuid;
use crate::domain::TaskEntry;

/// Struct, that used to get To do list from Database
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ToDoListEntry {
    pub id: Uuid,
    pub name: String
}
#[derive(Serialize, Deserialize, Debug)]
pub struct ToDoList {
    pub name: String,
    pub tasks: Vec<TaskEntry>,
}