use crate::helpers::spawn_app;
use yet_another_todo_list::domain::ToDoList;

#[tokio::test]
async fn get_todo_returns_200() {
    // Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    
    // Arrange
    let response = client
        .get(format!("{}/todo", app.address))
        .send()
        .await
        .expect("Failed to send request");
    
    // Asserts
    assert_eq!(response.status().as_u16(), 200);
}

#[tokio::test]
async fn get_todo_returns_empty_list() {
    // Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    // Arrange
    let response = client
        .get(format!("{}/todo", app.address))
        .send()
        .await
        .expect("Failed to send request");
    let text = response.text().await.expect("Failed to get text");
    let lists: Vec<ToDoList> = serde_json::from_str(&text).expect("Failed to make json");
    
    // Asserts
    assert_eq!(lists.len(), 0);
}