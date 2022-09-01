use actix_web::{HttpResponse, web};
use actix_web::web::Form;
use sqlx::PgPool;
use uuid::Uuid;
use chrono::Utc;

#[derive(serde::Deserialize)]
pub struct NewUserData {
    name: String,
    password: String,
}

pub async fn new_user(form: Form<NewUserData>, db_pool: web::Data<PgPool>) -> HttpResponse {
    match insert_user(&db_pool, form.0).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish()
    }
}

pub async fn insert_user(pool: &PgPool, new_user: NewUserData) -> Result<(), sqlx::Error> {
    sqlx::query!(r"
    INSERT INTO users (id, name, password, added_at)
    VALUES ($1, $2, $3, $4)
    ",
        Uuid::new_v4(),
        new_user.name,
        new_user.password,
        Utc::now()
    )
        .execute(pool)
        .await?;
    Ok(())
}