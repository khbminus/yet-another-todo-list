use std::net::TcpListener;
use yet_another_todo_list::startup::run;

pub struct TestApp {
    pub address: String,
}

pub async fn spawn_app() -> TestApp {

    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let address = format!("http://127.0.0.1:{}", port);

    let server =
        run(listener, "localhost".into()).expect("Failed to bind address");
    let _ = tokio::spawn(server);
    TestApp {
        address,
    }
}