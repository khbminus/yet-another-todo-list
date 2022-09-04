use actix_web::{HttpResponse, web};
use sqlx::{PgPool};
use crate::domain::{ToDoList, ToDoListEntry, TaskEntry};
use uuid::Uuid;
use chrono::Utc;
use serde::Deserialize;

pub async fn get_todo_lists(db_pool: web::Data<PgPool>) -> HttpResponse {
    let lists = match get_lists_from_db(&db_pool).await {
        Ok(lists) => lists,
        Err(_) => return HttpResponse::InternalServerError().finish()
    };
    HttpResponse::Ok().json(lists)
}

pub async fn get_lists_from_db(db_pool: &PgPool) -> Result<Vec<ToDoListEntry>, sqlx::Error> {
    sqlx::query_as!(ToDoListEntry,
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

pub async fn get_exact_list(id: web::Path<Uuid>, db_pool: web::Data<PgPool>) -> HttpResponse {
    let id = id.into_inner();
    match select_list(&id, &db_pool).await {
        Ok(list) => HttpResponse::Ok().json(list),
        Err(error) => match error {
            sqlx::Error::RowNotFound => HttpResponse::BadRequest().finish(),
            _ => HttpResponse::InternalServerError().finish()
        }
    }
}

async fn select_list(id: &Uuid, db_pool: &PgPool) -> Result<ToDoList, sqlx::Error> {
    let mut transaction = db_pool.begin().await?;
    let name = sqlx::query!("SELECT name FROM lists WHERE id = $1", id)
        .fetch_one(&mut transaction)
        .await?.name;
    let tasks = sqlx::query_as!(TaskEntry, r#"
    SELECT id, content, done 
    FROM tasks 
    WHERE list_id = $1 
    ORDER BY id
    "#, id)
        .fetch_all(&mut transaction)
        .await?;
    transaction.commit().await?;
    Ok(ToDoList {
        name,
        tasks
    })
}

