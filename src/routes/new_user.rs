use actix_web::HttpResponse;
use actix_web::web::Form;

#[derive(serde::Deserialize)]
pub struct NewUserData {
    name: String,
    password: String,
}

pub async fn new_user(_form: Form<NewUserData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}