use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = "http://localhost:4242/ip";
    match reqwest::get(path).await {
        Ok(response) => match response.text().await {
            Ok(text) => {
                println!("RESPONSE: {} bytes from {}", text.len(), path);
            }
            Err(_) => {
                println!("Error reading: {}", path);
            }
        },
        Err(_) => {
            println!("Error downloading {}", path);
        }
    }
    Ok(())
}
