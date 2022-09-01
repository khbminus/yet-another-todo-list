use actix_web::{HttpResponse, web};
use actix_web::web::Form;
use sqlx::PgPool;
use uuid::Uuid;
use chrono::Utc;
use crate::domain::{NewUser, UserName, UserPassword};
use std::convert::TryInto;

#[derive(serde::Deserialize)]
pub struct NewUserData {
    name: String,
    password: String,
}

impl TryFrom<NewUserData> for NewUser {
    type Error = String;
    
    fn try_from(value: NewUserData) -> Result<Self, Self::Error> {
        let name = UserName::parse(value.name)?;
        let password = UserPassword::parse(value.password)?;
        Ok(Self{name, password})
    }
}

pub async fn new_user(form: Form<NewUserData>, db_pool: web::Data<PgPool>) -> HttpResponse {
    let new_user = match form.0.try_into() {
        Ok(user) => user,
        Err(_) => return HttpResponse::BadRequest().finish()
    };
    match insert_user(&db_pool, &new_user).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish()
    }
}

pub async fn insert_user(pool: &PgPool, new_user: &NewUser) -> Result<(), sqlx::Error> {
    sqlx::query!(r"
    INSERT INTO users (id, name, password, added_at)
    VALUES ($1, $2, $3, $4)
    ",
        Uuid::new_v4(),
        new_user.name.as_ref(),
        new_user.password.as_ref(),
        Utc::now()
    )
        .execute(pool)
        .await?;
    Ok(())
}