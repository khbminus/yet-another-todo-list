use std::net::TcpListener;
use reqwest::Response;
use yet_another_todo_list::startup::run;
use yet_another_todo_list::configuration::{DatabaseSettings, get_configuration};
use sqlx::{Executor, PgPool, PgConnection, Connection};
use uuid::Uuid;
use yet_another_todo_list::domain::ToDoListEntry;

pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
}

impl TestApp {
    pub async fn add_user(&self, body: String) -> Response {
        reqwest::Client::new()
            .post(format!("{}/user/new", self.address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(body)
            .send()
            .await
            .expect("Failed to send request")
    }
    pub async fn get_todo_lists_response(&self) -> Response {
        reqwest::Client::new()
            .get(format!("{}/todo", self.address))
            .send()
            .await
            .expect("Failed to send request")
    }
    pub async fn get_todo_lists(&self) -> Vec<ToDoListEntry> {
        let response = self.get_todo_lists_response().await;
        response.json().await.expect("Failed to get lists")
    }
    pub async fn add_todo_list(&self, body: String) -> Response {
        reqwest::Client::new()
            .post(format!("{}/todo", self.address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(body)
            .send()
            .await
            .expect("Failed to send request")
    }
    pub async fn add_todo_list_by_name(&self, name: String) -> Response {
        self.add_todo_list(format!("name={}", name)).await
    }
    pub async fn get_list(&self, id: Uuid) -> Response {
        reqwest::Client::new()
            .get(format!("{}/todo/{}", self.address, id.to_string()))
            .send()
            .await
            .expect("Failed to send request")
    }
    
    pub async fn add_item(&self, id: Uuid, body: String) -> Response {
        reqwest::Client::new()
            .post(format!("{}/todo/{}", self.address, id.to_string()))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(body)
            .send()
            .await
            .expect("Failed to send request")
    }
    
    pub async fn make_complete(&self, list_id: Uuid, task_id: i32) -> Response {
        reqwest::Client::new()
            .post(format!("{}/todo/{}/{}/complete", self.address, list_id, task_id))
            .send()
            .await
            .expect("Failed to send requests")
    }
}

pub async fn spawn_app() -> TestApp {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let address = format!("http://127.0.0.1:{}", port);
    
    let mut configuration = get_configuration().expect("Failed to read configurarion");
    configuration.database.database_name = Uuid::new_v4().to_string();
    let connection_pool = configure_database(&configuration.database)
        .await;

    let server =
        run(listener, "localhost".into(), connection_pool.clone()).expect("Failed to bind address");
    let _ = tokio::spawn(server);
    TestApp {
        address,
        db_pool: connection_pool,
    }
}

pub async fn configure_database(config: &DatabaseSettings) -> PgPool {
    // Create database
    let mut connection = PgConnection::connect_with(&config.path_without_database())
        .await
        .expect("Failed to connect to Postgres");
    connection
        .execute(&*format!(r#"CREATE DATABASE "{}";"#, config.database_name))
        .await
        .expect("Failed to create database.");

    // Migrate database
    let connection_pool = PgPool::connect_with(config.path_with_database())
        .await
        .expect("Failed to connect to Postgres.");
    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to migrate the database");
    connection_pool
}