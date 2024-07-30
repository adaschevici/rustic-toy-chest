use inquire::Select;

use futures::future::BoxFuture;
use futures_util::{SinkExt, StreamExt};
use reqwest::Client;
use serde::Deserialize;
use serde_json::json;
use tokio::net::TcpStream;
use tokio_tungstenite::connect_async;
use tracing::info;
use tungstenite::protocol::Message;
use url::Url;

mod navigate_to_page;

mod map_tabs_to_struct;

mod handle_events;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().pretty().init();
    // Define the list of functions
    let functions: Vec<(&str, fn() -> BoxFuture<'static, ()>)> = vec![
        ("Navigate to example.com", || {
            Box::pin(navigate_to_page::navigate_to_page(None))
        }),
        ("List tabs", || Box::pin(map_tabs_to_struct::get_tabs())),
        ("Subscribe to event", || Box::pin(subscribe_to_event())),
    ];

    // Create a vector of function names
    let function_names: Vec<&str> = functions.iter().map(|(name, _)| *name).collect();

    // Prompt the user to select a function
    let selected_function = Select::new("Choose a function to execute:", function_names.clone())
        .with_starting_cursor(function_names.len() - 1)
        .prompt()
        .expect("Failed to read input");

    // Find and execute the corresponding function
    for (name, function) in functions {
        if name == selected_function {
            function().await;
        }
    }
}
