use std::net::TcpListener;
use actix_web::dev::Server;
use actix_web::{App, HttpServer, web};
use crate::routes::health_check;

pub struct ApplicationBaseUrl(pub String);

pub fn run(
    listener: TcpListener,
    base_url: String
) -> Result<Server, std::io::Error> {
    let base_url = ApplicationBaseUrl(base_url);
    let server = HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
    })
        .listen(listener)?
        .run();
    Ok(server)
}