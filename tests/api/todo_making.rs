use uuid::Uuid;
use yet_another_todo_list::domain::ToDoList;
use crate::helpers::{spawn_app, TestApp};

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

async fn prepare_list(app: &TestApp) -> Uuid {
    app.add_todo_list_by_name("name".into()).await;
    app.get_todo_lists().await[0].id
}

#[tokio::test]
async fn add_new_item_returns_200() {
    let app = spawn_app().await;
    let body = "content=Make%20project%20done%20%26%20touch%20the%20grass%21";
    let list_id = prepare_list(&app).await;
    
    let response = app.add_item(list_id, body.into()).await;
    
    assert_eq!(response.status().as_u16(), 200);
}

#[tokio::test]
async fn add_new_item_returns_400_on_empty_form() {
    let app = spawn_app().await;
    let body = "content=";
    let list_id = prepare_list(&app).await;

    let response = app.add_item(list_id, body.into()).await;
    
    assert_eq!(response.status().as_u16(), 400);
}

#[tokio::test]
async fn add_new_item_actually_adds() {
    let app = spawn_app().await;
    let body = "content=Hallo";
    let list_id = prepare_list(&app).await;
    
    app.add_item(list_id, body.into()).await;
    
    let saved = sqlx::query!("SELECT content, done FROM tasks")
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to fetch");
    
    assert!(!saved.done);
    assert_eq!(saved.content, "Hallo");
}

#[tokio::test]
async fn get_list_actually_return_task() {
    let app = spawn_app().await;
    let body = "content=Hallo";
    let list_id = prepare_list(&app).await;

    app.add_item(list_id, body.into()).await;
    
    let list: ToDoList = app.get_list(list_id).await.json().await.expect("Failed to get list");
    
    assert_eq!(list.tasks.len(), 1);
    assert!(!list.tasks[0].done);
    assert_eq!(list.tasks[0].content, "Hallo");
}

#[tokio::test]
async fn add_new_item_preserves_order() {
    let app = spawn_app().await;
    let list_id = prepare_list(&app).await;
    
    for i in 1..10 {
        app.add_item(list_id, format!("content=Task{}", i)).await;
    }
    
    let list: ToDoList = app.get_list(list_id).await.json().await.expect("Failed to get list");
    
    for i in 1..10 {
        assert_eq!(list.tasks[i - 1].content, format!("Task{}", i));
    }
}

#[tokio::test]
async fn add_new_item_adds_to_correct_list() {
    let app = spawn_app().await;
    let list_id1 = prepare_list(&app).await;
    app.add_todo_list_by_name("name2".into()).await;
    let lists = app.get_todo_lists().await;
    let list_id2 = lists[1].id;
    
    app.add_item(list_id1, "content=List1".into()).await;
    app.add_item(list_id2, "content=List2".into()).await;
    
    let list1: ToDoList = app.get_list(list_id1).await.json().await.expect("Failed to get list");
    let list2: ToDoList = app.get_list(list_id2).await.json().await.expect("Failed to get list");
    
    assert_eq!(list1.tasks.len(), 1);
    assert_eq!(list1.tasks[0].content, "List1");
    assert_eq!(list2.tasks[0].content, "List2");
}