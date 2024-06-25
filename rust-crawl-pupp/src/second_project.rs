use chromiumoxide::browser::Browser;

pub async fn grab_root_content(
    browser: &mut Browser,
) -> Result<String, Box<dyn std::error::Error>> {
    let page = browser
        .new_page("https://scrapingclub.com/exercise/list_infinite_scroll/")
        .await?;
    let content = page.content().await?;
    Ok(content)
}
