use reqwest::Client;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Tab {
    id: String,
    title: String,
    url: String,
    #[serde(rename = "webSocketDebuggerUrl")]
    websocket_debugger_url: Option<String>,
}

pub async fn get_tabs() {
    // Connect to the WebSocket server
    let client = Client::new();

    let tabs: Vec<Tab> = client
        .get("http://localhost:9222/json")
        .send()
        .await
        .expect("Failed to send request")
        .json()
        .await
        .expect("Failed to parse JSON");

    // Print the tab IDs and their URLs
    for tab in tabs.iter() {
        println!("Tab ID: {}, Title: {}, URL: {}", tab.id, tab.title, tab.url);
        if let Some(ws_url) = &tab.websocket_debugger_url {
            println!("WebSocket Debugger URL: {}", ws_url);
        }
    }
}
