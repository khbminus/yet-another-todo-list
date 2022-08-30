use std::net::TcpListener;
use yet_another_todo_list::startup::run;
use yet_another_todo_list::configuration::get_configuration;
use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let configuration = get_configuration().expect("Failed to read configuration");
    let db_pool = PgPoolOptions::new()
        .acquire_timeout(std::time::Duration::from_secs(2))
        .connect_with(configuration.database.path_with_database())
        .await
        .expect("Failed to connect to database");

    let address = format!("{}:{}", configuration.application.host, configuration.application.port);
    let listener = TcpListener::bind(address)?;
    run(listener, "localhost".into(), db_pool)?.await?;
    Ok(())
}