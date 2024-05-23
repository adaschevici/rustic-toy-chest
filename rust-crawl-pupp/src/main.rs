use futures::StreamExt;
use std::{thread, time};

use chromiumoxide::browser::{Browser, BrowserConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let (mut browser, mut handler) =
        Browser::launch(BrowserConfig::builder().with_head().build()?).await?;

    let handle = tokio::task::spawn(async move {
        loop {
            match handler.next().await {
                Some(h) => match h {
                    Ok(_) => continue,
                    Err(_) => break,
                },
                None => break,
            }
        }
    });

    let page = browser
        .new_page("https://doc.rust-lang.org/std/thread/fn.sleep.html")
        .await?;
    // let ten_millis = time::Duration::from_millis(5000);
    // let now = time::Instant::now();

    // thread::sleep(ten_millis);

    // page.find_element("a.cdx-button--fake-button--enabled")
    //     .await?
    //     .click()
    //     .await?;

    page.find_element("input.search-input")
        .await?
        .click()
        .await?
        .type_str("Rust programming language")
        .await?
        .press_key("Enter")
        .await?;

    // let _html = page.wait_for_navigation().await?.content().await?;

    browser.close().await?;
    handle.await?;
    Ok(())
}
