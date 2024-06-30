use tokio::net::TcpStream;
use tokio_tungstenite::connect_async;
use tungstenite::protocol::Message;
use url::Url;

#[tokio::main]
async fn main() {
    // Replace with the WebSocket URL of the Chrome browser.
    let websocket_url = "ws://localhost:9222/devtools/page/your_page_id";
    let url = Url::parse(websocket_url).expect("Invalid WebSocket URL");

    // Connect to the WebSocket server
    let (ws_stream, _) = connect_async(url).await.expect("Failed to connect");

    // Split the stream into a sink (to send messages) and a stream (to receive messages)
    let (mut write, mut read) = ws_stream.split();

    // Send a message to the WebSocket server
    write.send(Message::Text("Hello WebSocket".into())).await.expect("Failed to send message");

    // Read a message from the WebSocket server
    if let Some(message) = read.next().await {
        match message {
            Ok(msg) => {
                match msg {
                    Message::Text(text) => println!("Received: {}", text),
                    Message::Binary(bin) => println!("Received binary data: {:?}", bin),
                    _ => (),
                }
            }
            Err(e) => println!("Error receiving message: {}", e),
        }
    }
}
