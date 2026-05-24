/// Run the server locally first: `cargo run -p station-service`
/// These tests are ignored in CI because they require a running server.

#[ignore = "requires running server at localhost:8000"]
#[tokio::test]
async fn test_health_live() {
    let res = reqwest::Client::new()
        .get("http://localhost:8000/health/live")
        .send()
        .await
        .expect("Server should be running at localhost:8000");
    assert_eq!(res.status(), 200);
}

#[ignore = "requires running server at localhost:8000"]
#[tokio::test]
async fn test_health_ready() {
    let res = reqwest::Client::new()
        .get("http://localhost:8000/health/ready")
        .send()
        .await
        .expect("Server should be running at localhost:8000");
    assert!(res.status() == 200 || res.status() == 503);
}
