use crate::helpers::spawn_app;

#[tokio::test]
async fn user_new_return_200_on_valid_form() {
    // Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    // Act
    let form = "name=khb&password=aboba";
    let response = client
        .post(format!("{}/user/new", app.address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(form)
        .send()
        .await
        .expect("Failed to send request");

    // Asserts
    assert_eq!(response.status().as_u16(), 200);
}
#[tokio::test]
async fn user_new_actually_add_user() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let form = "name=khb&password=aboba";
    client
        .post(format!("{}/user/new", app.address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(form)
        .send()
        .await
        .expect("Failed to send request");
    
    let inserted = sqlx::query!("SELECT name, password FROM users")
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to fetch user");
    
    assert_eq!(inserted.name, "khb");
    assert_eq!(inserted.password, "aboba");
}

#[tokio::test]
async fn user_new_returns_400_when_something_is_missing() {
    // Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=khb", "Missing password"),
        ("password=aboba", "Missing name"),
        ("", "Missing name and password"),
    ];

    // Act
    for (body, case_name) in test_cases {
        let response = client
            .post(format!("{}/user/new", app.address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(body)
            .send()
            .await
            .expect("Failed to send request");
        assert_eq!(response.status().as_u16(), 400, "Case \"{}\" failed and returned not 400 Bad Request code", case_name);
    }
}

#[tokio::test]
async fn user_with_wrong_data_returns_400() {
    // Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=ХБ&password=1234567", "Name contains non-ASCII chars"),
        ("name=&password=", "Empty name"),
        ("name=khb&password=12345", "Password is too short")
    ];
    
    for (body, case_name) in  test_cases {
        let response = client
            .post(format!("{}/user/new", app.address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(body)
            .send()
            .await
            .expect("Failed to send request");
        assert_eq!(response.status().as_u16(), 400, "Case \"{}\" failed and returned not 400 Bad Request code", case_name);
    }
}