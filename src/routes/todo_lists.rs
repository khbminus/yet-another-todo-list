use actix_web::{HttpResponse, web};
use sqlx::PgPool;
use crate::domain::ToDoList;

pub async fn get_todo_lists(db_pool: web::Data<PgPool>) -> HttpResponse {
    let lists = match get_lists_from_db(&db_pool).await {
        Ok(lists) => lists,
        Err(_) => return HttpResponse::InternalServerError().finish()
    };
    HttpResponse::Ok().json(lists)
}

pub async fn get_lists_from_db(db_pool: &PgPool) -> Result<Vec<ToDoList>, sqlx::Error> {
    sqlx::query_as!(ToDoList,
        r#"
    SELECT id, name FROM lists
    "#)
        .fetch_all(db_pool)
        .await
}