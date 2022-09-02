use serde::{Serialize, Deserialize};
use uuid::Uuid;

/// Struct, that used to get To do list from Database
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ToDoList {
    pub id: Uuid,
    pub name: String
}

impl ToDoList {
    pub fn name(&self) -> &String {
        &self.name
    }
    pub fn list_id(&self) -> &Uuid {
        &self.id
    }
}