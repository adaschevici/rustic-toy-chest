use chromiumoxide::page::ScreenshotParams;
use chromiumoxide_cdp::cdp::browser_protocol::page::CaptureScreenshotFormat;

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
