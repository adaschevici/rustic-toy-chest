use chromiumoxide::browser::{Browser, BrowserConfig};

pub async fn crawl(browser: &mut Browser) -> Result<String, Box<dyn std::error::Error>> {
    let page = browser.new_page("https://en.wikipedia.org").await?;
    page.find_element("input[name='search']")
        .await?
        .click()
        .await?
        .type_str("Rust programming language")
        .await?
        .press_key("Enter")
        .await?;
    let _html = page.wait_for_navigation().await?.content().await?;
    Ok(_html)
}
