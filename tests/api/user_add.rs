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