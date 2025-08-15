use tokio::net::TcpListener;
use zero2prod_backend;

#[tokio::test]
async fn test_signup() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();

    tokio::spawn(async move {
        zero2prod_backend::run(listener).await.unwrap();
    });

    let client = reqwest::Client::new();
    let response = client
        .post(&format!("http://{}/signup", addr))
        .json(&serde_json::json!({
            "email": "test@example.com",
            "password": "password"
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), reqwest::StatusCode::OK);
}
