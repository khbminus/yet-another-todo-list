use uuid::Uuid;
use yet_another_todo_list::domain::ToDoList;
use crate::helpers::spawn_app;

#[tokio::test]
async fn todo_with_valid_id_returns_200() {
    let app = spawn_app().await;
    app.add_todo_list_by_name("name".into()).await;
    
    let list_id = app.get_todo_lists().await[0].id;
    let response = app.get_list(list_id).await;
    
    assert_eq!(response.status().as_u16(), 200);
}

#[tokio::test]
async fn todo_with_invalid_id_returns_400() {
    let app = spawn_app().await;
    app.add_todo_list_by_name("name".into()).await;

    let list_id = Uuid::from_u128(app.get_todo_lists().await[0].id.as_u128() + 1);
    let response = app.get_list(list_id).await;
    
    assert_eq!(response.status().as_u16(), 400);
}

#[tokio::test]
async fn get_list_returns_correct_list() {
    let app = spawn_app().await;
    app.add_todo_list_by_name("name1".into()).await;
    app.add_todo_list_by_name("name2".into()).await;
    
    let lists = app.get_todo_lists().await;
    let list1: ToDoList = app.get_list(lists[0].id).await.json().await.expect("Failed to decode");
    let list2: ToDoList = app.get_list(lists[1].id).await.json().await.expect("Failed to decode");
    
    assert_eq!(list1.name, lists[0].name);
    assert_eq!(list2.name, lists[1].name);
    
}