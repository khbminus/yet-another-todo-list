use crate::helpers::spawn_app;
use yet_another_todo_list::domain::ToDoList;

#[tokio::test]
async fn get_todo_returns_200() {
    // Arrange
    let app = spawn_app().await;
    // Act
    let response = app.get_todo_lists().await;
    // Asserts
    assert_eq!(response.status().as_u16(), 200);
}

#[tokio::test]
async fn get_todo_returns_empty_list() {
    // Arrange
    let app = spawn_app().await;

    // Act
    let response = app.get_todo_lists().await;
    let text = response.text().await.expect("Failed to get text");
    let lists: Vec<ToDoList> = serde_json::from_str(&text).expect("Failed to make json");

    // Asserts
    assert_eq!(lists.len(), 0);
}

#[tokio::test]
async fn post_todo_returns_200_on_correct_data() {
    // Arrange
    let app = spawn_app().await;
    let body = "name=Aboba";

    // Act
    let response = app.add_todo_list(body.to_owned()).await;

    // Asserts

    assert_eq!(response.status().as_u16(), 200);
}

#[tokio::test]
async fn post_todo_actually_adds_to_db() {
    let app = spawn_app().await;
    let body = "name=Aboba";

    // Act
    let response = app.add_todo_list(body.to_owned()).await;
    
    // Asserts
    let saved = sqlx::query!("SELECT name from lists")
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to fetch SQL query");
    
    assert_eq!(saved.name, "Aboba");
}

#[tokio::test]
async fn get_contain_list_after_post() {
    let app = spawn_app().await;
    let body = "name=Aboba";
    
    app.add_todo_list(body.to_owned()).await;
    
    let response = app.get_todo_lists().await;
    let lists: Vec<ToDoList> = response.json().await.expect("Failed to json");
    
    assert_eq!(lists.len(), 1);
    assert_eq!(lists[0].name, "Aboba");
}