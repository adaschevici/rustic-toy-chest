use chromiumoxide::browser::Browser;

pub async fn grab_root_content(
    browser: &mut Browser,
) -> Result<String, Box<dyn std::error::Error>> {
    let page = browser.new_page("https://www.rust-lang.org").await?;
    let content = page.content().await?;
    Ok(content)
}
