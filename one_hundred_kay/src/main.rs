use futures::future::join_all;
use futures::stream::StreamExt;
use rand::Rng;
use reqwest::Client;
use std::collections::HashMap;
use std::time::Duration;
use std::{
    iter,
    iter::{FromIterator, RepeatWith, Take},
};
use tokio::task::JoinHandle;

fn generate_random_delay_urls(
    min: i32,
    max: i32,
    limit: usize,
) -> Take<RepeatWith<impl FnMut() -> String>> {
    iter::repeat_with(move || {
        let mut rng = rand::thread_rng(); // Get a random number generator.
        let random_delay = rng.gen_range(min..max); // Generate a random number within the range.
        format!("http://localhost:4242/delay/{}", random_delay)
    })
    .take(limit)
}
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let paths = generate_random_delay_urls(1, 10, 10);
    let path = "http://localhost:4242/ip";
    let client = Client::builder().timeout(Duration::from_secs(5)).build()?;
    let mut tasks: Vec<JoinHandle<Result<String, reqwest::Error>>> = vec![];
    let fetches = futures::stream::iter(paths.map(|path| {
        let client = client.clone();
        async move {
            match client.get(&path).send().await {
                Ok(response) => match response.text().await {
                    Ok(text) => {
                        println!("RESPONSE: {} bytes from {}", text.len(), path);
                    }
                    Err(_) => println!("ERROR reading {}", path),
                },
                Err(_) => println!("ERROR fetching {}", path),
            }
        }
    }))
    .buffer_unordered(5)
    .collect::<Vec<()>>();
    println!("Waiting...");
    fetches.await;
    Ok(())
}
