use actix_web::{HttpResponse, web};
use sqlx::PgPool;
use crate::domain::ToDoList;
use uuid::Uuid;
use chrono::Utc;
use serde::Deserialize;

pub async fn get_todo_lists(db_pool: web::Data<PgPool>) -> HttpResponse {
    let lists = match get_lists_from_db(&db_pool).await {
        Ok(lists) => lists,
        Err(e) => {
            dbg!(e);
            return HttpResponse::InternalServerError().finish();
        }
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

#[derive(Deserialize)]
pub struct NewListData {
    pub name: String,
}

pub async fn post_new_list(form: web::Form<NewListData>, db_pool: web::Data<PgPool>) -> HttpResponse {
    match insert_list(&form.name, &db_pool).await {
        Err(_) => HttpResponse::InternalServerError().finish(),
        Ok(_) => HttpResponse::Ok().finish()
    }
}

async fn insert_list(name: &String, db_pool: &PgPool) -> Result<(), sqlx::Error> {
    sqlx::query!(r#"
        INSERT INTO lists (id, name, added_at)
        VALUES ($1, $2, $3)
    "#,
        Uuid::new_v4(),
        name,
        Utc::now()
    )
        .execute(db_pool)
        .await?;
    Ok(())
}