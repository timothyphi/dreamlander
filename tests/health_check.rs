use dreamlander::run;
use reqwest;

#[tokio::test]
async fn health_check_works() {
    assert_eq!(0, 1);
    // spawn_app().await.expect("failed to spawn our app.");
    let client = reqwest::Client::new();

    let response = client
        .get("http://127.0.0.1:8080/health_check")
        .send()
        .await
        .expect("failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

async fn spawn_app() -> Result<(), std::io::Error> {
    run().await
}
