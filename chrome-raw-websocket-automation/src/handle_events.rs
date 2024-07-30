use futures_util::{SinkExt, StreamExt};
use tokio_tungstenite::connect_async;
use tracing::info;
use tungstenite::protocol::Message;
use url::Url;

async fn subscribe_to_event() {
    let websocket_url = "ws://127.0.0.1:9222/devtools/browser/443ae8d0-81a8-4340-8961-1f15d019ba76";
    let url = Url::parse(websocket_url).expect("Invalid WebSocket URL");
    info!("Connecting to {}", url);
    // Connect to the WebSocket server
    let (mut ws_stream, _) = connect_async(websocket_url)
        .await
        .expect("Failed to connect");
    let (mut write, mut read) = ws_stream.split();
    write
        .send(Message::Text(
            json!({
                "id": 1,
                "method": "Target.getTargets"
            })
            .to_string(),
        ))
        .await;
    let message = read.next().await.ok_or("No response");
    info!("Received message: {:?}", message);
    let response: serde_json::Value =
        serde_json::from_str(message.unwrap().unwrap().to_text().unwrap())
            .expect("Unable to parse JSON");
    info!("Response: {:?}", response);
    let target = response["result"]["targetInfos"]
        .as_array()
        .and_then(|arr| arr.iter().find(|target| target["type"] == "page"))
        .ok_or("No page target found");
    info!("Target: {:?}", target);

    let target_infos = response["result"]["targetInfos"]
        .as_array()
        .ok_or("No targetInfos array");

    let target_infos = target_infos.unwrap();

    // Iterate over each target and print the URL
    for target in target_infos {
        if let Some(url) = target["url"].as_str() {
            info!("Target URL: {}", url);
            info!("Target ID: {}", target["targetId"]);
        } else {
            info!("No URL found for target");
        }
    }
}
