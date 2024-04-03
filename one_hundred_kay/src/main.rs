use futures::future::join_all;
use rand::Rng;
use std::collections::HashMap;
use std::{iter, iter::RepeatWith};
use tokio::task::JoinHandle;

fn generate_random_delay_urls(min: i32, max: i32) -> RepeatWith<impl FnMut() -> String> {
    let infinite_url_list = iter::repeat_with(|| {
        let mut rng = rand::thread_rng(); // Get a random number generator.
        let random_delay = rng.gen_range(min..max); // Generate a random number within the range.
        let url = format!("http://localhost:4242/ip?delay={}", random_delay);
        url
    });
    infinite_url_list
}
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let paths = vec![
        "http://localhost:4242/ip".to_string(),
        "http://localhost:4242/ip".to_string(),
        "http://localhost:4242/ip".to_string(),
    ];
    let path = "http://localhost:4242/ip";
    let mut tasks: Vec<JoinHandle<Result<String, reqwest::Error>>> = vec![];
    for path in paths {
        // Copy each path into a new string
        // that can be used in the task closure
        let path = path.clone();

        // Create tokio tasks for each path
        tasks.push(tokio::spawn(async move {
            let result = match reqwest::get(&path).await {
                Ok(resp) => match resp.text().await {
                    Ok(text) => {
                        println!("RESPONSE: {} bytes from {}", text.len(), path);
                        Ok(text)
                    }
                    Err(e) => {
                        println!("ERROR reading {}", path);
                        Err(e)
                    }
                },
                Err(e) => {
                    println!("ERROR downloading {}", path);
                    Err(e)
                }
            };
            result
        }));
    }
    // Wait for them all to finish
    println!("Started {} tasks. Waiting...", tasks.len());
    join_all(tasks).await;
    Ok(())
}
