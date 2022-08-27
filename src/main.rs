use std::net::TcpListener;
use yet_another_todo_list::startup::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let address: String = "127.0.0.1:8000".into();
    let listener = TcpListener::bind(address)?;
    run(listener, "localhost".into())?.await?;
    Ok(())
}