use chromiumoxide::page::ScreenshotParams;
use chromiumoxide_cdp::cdp::browser_protocol::page::{
    CaptureScreenshotFormat, CaptureScreenshotParams,
};

use chromiumoxide::Browser;

pub async fn capture_full_page_screenshot(
    browser: &mut Browser,
) -> Result<(), Box<dyn std::error::Error>> {
    let page = browser
        .new_page("https://scrapingclub.com/exercise/list_infinite_scroll/")
        .await?;
    let screenshot = page
        .save_screenshot(
            ScreenshotParams::builder()
                .format(CaptureScreenshotFormat::Png)
                .full_page(true)
                .omit_background(true)
                .build(),
            "example.png",
        )
        .await?;
    Ok(())
}

pub async fn capture_selector_screenshot(
    browser: &mut Browser,
    selector: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let page = browser
        .new_page("https://scrapingclub.com/exercise/list_infinite_scroll/")
        .await?;
    let element_handle = page.find_element(selector).await?;
    element_handle.scroll_into_view().await?;
    let screenshot = element_handle
        .save_screenshot(CaptureScreenshotFormat::Png, "example-selector.png")
        .await?;
    Ok(())
}
