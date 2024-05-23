use futures::StreamExt;

use chromiumoxide::{Browser, BrowserConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let browser = Browser::connect(BrowserConfig::builder().build().await.unwrap())
        .await
        .unwrap();

    let tab = browser.new_tab().await.unwrap();

    let response = tab
        .navigate_to("https://www.rust-lang.org/")
        .await
        .unwrap();

    let title = response.title().await.unwrap();
    println!("Title: {}", title);

    let elements = tab.find_elements("a").await.unwrap();
    let mut count = 0;
    while let Some(element) = elements.next().await {
        count += 1;
        println!("Element: {:?}", element);
    }
    println!("Found {} elements", count);

    tab.close().await.unwrap();
    browser.close().await.unwrap();
}
