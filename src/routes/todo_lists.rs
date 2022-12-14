use actix_web::{HttpResponse, web};
use sqlx::{PgPool};
use crate::domain::{ToDoList, ToDoListEntry, TaskEntry, ToDoListName};
use uuid::Uuid;
use chrono::Utc;
use serde::Deserialize;

pub async fn get_todo_lists(db_pool: web::Data<PgPool>) -> HttpResponse {
    if let Ok(lists) = get_lists_from_db(&db_pool).await {
        HttpResponse::Ok().json(lists)
    } else {
        HttpResponse::InternalServerError().finish()
    }
}

pub async fn get_lists_from_db(db_pool: &PgPool) -> Result<Vec<ToDoListEntry>, sqlx::Error> {
    sqlx::query_as!(ToDoListEntry,
        r#"
    SELECT id, name FROM lists
    ORDER BY added_at
    "#)
        .fetch_all(db_pool)
        .await
}

#[derive(Deserialize)]
pub struct NewListData {
    pub name: String,
}

pub async fn post_new_list(form: web::Form<NewListData>, db_pool: web::Data<PgPool>) -> HttpResponse {
    let name = match ToDoListName::parse(form.0.name) {
        Ok(res) => res,
        Err(_) => { return HttpResponse::BadRequest().finish(); }
    };
    match insert_list(name, &db_pool).await {
        Err(_) => HttpResponse::InternalServerError().finish(),
        Ok(_) => HttpResponse::Ok().finish()
    }
}

async fn insert_list(name: ToDoListName, db_pool: &PgPool) -> Result<(), sqlx::Error> {
    sqlx::query!(r#"
        INSERT INTO lists (id, name, added_at)
        VALUES ($1, $2, $3)
    "#,
        Uuid::new_v4(),
        name.as_ref(),
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
        Err(error) =>
            if let sqlx::Error::RowNotFound = error { HttpResponse::BadRequest().finish() } else { HttpResponse::InternalServerError().finish() }
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
        tasks,
    })
}

#[derive(Deserialize)]
pub struct NewTaskData {
    content: String,
}

pub async fn add_new_task(
    path: web::Path<Uuid>,
    new_task: web::Form<NewTaskData>,
    db_pool: web::Data<PgPool>,
) -> HttpResponse {
    let id = path.into_inner();
    if insert_new_task(&id, &new_task, &db_pool).await.is_ok() {
        HttpResponse::Ok().finish()
    } else {
        HttpResponse::InternalServerError().finish()
    }
}

async fn insert_new_task(
    id: &Uuid,
    new_task: &NewTaskData,
    db_pool: &PgPool,
) -> Result<(), sqlx::Error> {
    sqlx::query!(r#"
    INSERT INTO tasks(id, list_id, content, done)
    VALUES (DEFAULT, $1, $2, $3)
    "#,
        id,
        new_task.content,
        false)
        .execute(db_pool)
        .await?;

    Ok(())
}

#[derive(Deserialize)]
pub struct MakeDoneData {
    pub list_id: Uuid,
    pub task_id: i32,
}

pub async fn make_task_done(path: web::Path<MakeDoneData>, db_pool: web::Data<PgPool>) -> HttpResponse {
    let path = path.into_inner();
    match flip_doneness(&path.list_id, path.task_id, &db_pool).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            match e {
                sqlx::Error::RowNotFound => HttpResponse::BadRequest().finish(),
                _ => HttpResponse::InternalServerError().finish()
            }
        }
    }
}

async fn flip_doneness(list_id: &Uuid, task_id: i32, db_pool: &PgPool) -> Result<(), sqlx::Error> {
    sqlx::query!(r#"
        UPDATE tasks
        SET done = NOT done
        WHERE list_id = $1 AND id = $2
    "#, list_id, task_id)
        .execute(db_pool)
        .await?;
    Ok(())
}


