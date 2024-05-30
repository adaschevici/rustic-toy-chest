use cdp_rs::{parms, CdpClient};

fn main() {
    // Connect to the first tab of your open Chrome instance
    // Connect to first tab to a chrome instance running on a non-default remote-debugging-port
    let mut cdp = CdpClient::new().connect_to_tab(0);

    // Send a message so we can recieve DOM events
    cdp.clone().expect("Failed").send("DOM.enable", parms!());
    while let Ok(m) = cdp.expect("Failed again").wait_message() {
        // Print out all messages recieved
        print!("Recieved: {}", m)
    }
    // Check cookies
}
