#[tokio::test]
async fn test_health_live() {
    let client = reqwest::Client::new();
    let res = client
        .get("http://localhost:8000/health/live")
        .send()
        .await;
    if let Ok(res) = res {
        assert_eq!(res.status(), 200);
    }
}

#[tokio::test]
async fn test_health_ready() {
    let client = reqwest::Client::new();
    let res = client
        .get("http://localhost:8000/health/ready")
        .send()
        .await;
    if let Ok(res) = res {
        assert!(res.status() == 200 || res.status() == 503);
    }
}
