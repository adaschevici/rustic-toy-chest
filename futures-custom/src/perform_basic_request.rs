use reqwest::Error;
use tracing::info;

pub async fn perform_basic_request() {
    let response = reqwest::get("https://jsonplaceholder.typicode.com/posts/1")
        .await
        .unwrap() // Wait for the request to complete and propagate any errors
        .text() // Convert the response body to text
        .await; // Wait for the text conversion and propagate any errors
    info!(
        "Response: {}",
        response.unwrap_or_else(|_| "Failed to get response".to_string())
    );
}
