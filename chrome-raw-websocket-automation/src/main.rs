use inquire::Select;

use futures::future::BoxFuture;
use futures_util::{SinkExt, StreamExt};
use serde_json::json;
use tokio::net::TcpStream;
use tokio_tungstenite::connect_async;
use tracing::info;
use tungstenite::protocol::Message;
use url::Url;

#[derive(Debug, Deserialize)]
struct Tab {
    id: String,
    title: String,
    url: String,
    #[serde(rename = "webSocketDebuggerUrl")]
    websocket_debugger_url: String,
}

async fn navigate_to_page() {
    // Replace with the WebSocket URL of the Chrome browser.
    let websocket_url = "ws://localhost:9222/devtools/page/F179AA99B5124885127674A3853BE659";
    let url = Url::parse(websocket_url).expect("Invalid WebSocket URL");
    info!("Connecting to {}", url);

    // Connect to the WebSocket server
    let (ws_stream, _) = connect_async(websocket_url)
        .await
        .expect("Failed to connect");
    //
    // // Split the stream into a sink (to send messages) and a stream (to receive messages)
    let (mut write, mut read) = ws_stream.split();
    let cdp_message = json!({
        "id": 1,
        "method": "Page.navigate",
        "params": {
            "url": "https://www.example.com"
        }
    });
    //
    // // Send a message to the WebSocket server
    write
        .send(Message::Text(cdp_message.to_string()))
        .await
        .expect("Failed to send message");
    //
    // // Read a message from the WebSocket server
    if let Some(message) = read.next().await {
        match message {
            Ok(msg) => match msg {
                Message::Text(text) => println!("Received: {}", text),
                Message::Binary(bin) => println!("Received binary data: {:?}", bin),
                _ => (),
            },
            Err(e) => println!("Error receiving message: {}", e),
        }
    }
}

async fn subscribe_to_event() {}

async fn list_targets() {}

async fn get_tabs() {
    // Connect to the WebSocket server
    let client = Client::new();

    let tabs: Vec<Tab> = client
        .get("http://localhost:9222/json")
        .send()
        .await?
        .json()
        .await?;

    // Print the tab IDs and their URLs
    for tab in tabs.iter() {
        println!("Tab ID: {}, Title: {}, URL: {}", tab.id, tab.title, tab.url);
        if let Some(ws_url) = &tab.websocket_debugger_url {
            println!("WebSocket Debugger URL: {}", ws_url);
        }
    }
}
#[tokio::main]
async fn main() {
    // Define the list of functions
    let functions: Vec<(&str, fn() -> BoxFuture<'static, ()>)> = vec![
        ("Navigate to example.com", || Box::pin(navigate_to_page())),
        ("List tabs", || Box::pin(get_tabs())),
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

fn greet() {
    println!("Hello, world!");
}

fn add() {
    let a = 2;
    let b = 3;
    println!("{} + {} = {}", a, b, a + b);
}

fn subtract() {
    let a = 5;
    let b = 3;
    println!("{} - {} = {}", a, b, a - b);
}

fn multiply() {
    let a = 4;
    let b = 3;
    println!("{} * {} = {}", a, b, a * b);
}
