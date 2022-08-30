use std::net::TcpListener;
use actix_web::dev::Server;
use actix_web::{App, HttpServer, web};
use crate::routes::{health_check, new_user};
use sqlx::PgPool;
use actix_web::web::Data;

pub struct ApplicationBaseUrl(pub String);

pub fn run(
    listener: TcpListener,
    base_url: String,
    db_pool: PgPool
) -> Result<Server, std::io::Error> {
    let base_url = ApplicationBaseUrl(base_url);
    let db_pool = Data::new(db_pool);
    let server = HttpServer::new(move || {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/user/new", web::post().to(new_user))
            .app_data(db_pool.clone())
    })
        .listen(listener)?
        .run();
    Ok(server)
}