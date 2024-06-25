use chromiumoxide::browser::{Browser, BrowserConfig};

pub async fn spoof_user_agent(browser: &mut Browser) -> Result<String, Box<dyn std::error::Error>> {
    let page = browser.new_page("about:blank").await?;
    page.set_user_agent(
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) \
         Chrome/58.0.3029.110 Safari/537.36",
    )
    .await?;
    page.goto("https://www.whatismybrowser.com/detect/what-is-my-user-agent")
        .await?;
    let detected_ua = page
        .find_element("div.detected_result > div#detected_value")
        .await?
        .inner_text()
        .await?
        .unwrap();
    Ok(detected_ua)
}
