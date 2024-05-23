use futures::StreamExt;
use std::{thread, time};

use chromiumoxide::browser::{Browser, BrowserConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let (browser, mut handler) =
        Browser::launch(BrowserConfig::builder().with_head().build()?).await?;

    let handle = tokio::task::spawn(async move {
        loop {
            let _ = handler.next().await.unwrap();
        }
    });

    let page = browser.new_page("https://en.wikipedia.org").await?;
    let ten_millis = time::Duration::from_millis(50000);
    let now = time::Instant::now();

    thread::sleep(ten_millis);

    page.find_element("input.cdx-text-input__input")
        .await?
        .click()
        .await?
        .type_str("Rust programming language")
        .await?
        .press_key("Enter")
        .await?;

    let _html = page.wait_for_navigation().await?.content().await?;

    handle.await?;
    Ok(())
}
