use std::net::TcpListener;
use actix_web::dev::Server;
use actix_web::{App, HttpServer, web};
use crate::routes::{health_check, new_user, get_todo_lists, post_new_list, get_exact_list, add_new_task, make_task_done};
use sqlx::PgPool;
use actix_web::web::Data;

pub struct ApplicationBaseUrl(pub String);

pub fn run(
    listener: TcpListener,
    base_url: String,
    db_pool: PgPool,
) -> Result<Server, std::io::Error> {
    let _base_url = ApplicationBaseUrl(base_url);
    let db_pool = Data::new(db_pool);
    let server = HttpServer::new(move || {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/user/new", web::post().to(new_user))
            .route("/todo", web::get().to(get_todo_lists))
            .route("/todo", web::post().to(post_new_list))
            .route("/todo/{id}", web::get().to(get_exact_list))
            .route("/todo/{id}", web::post().to(add_new_task))
            .route("/todo/{list_id}/{task_id}/complete", web::post().to(make_task_done))
            .app_data(db_pool.clone())
    })
        .listen(listener)?
        .run();
    Ok(server)
}