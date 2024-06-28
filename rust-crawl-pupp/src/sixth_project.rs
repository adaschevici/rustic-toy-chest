use chromiumoxide::browser::Browser;
use chromiumoxide::element::Element;
use chromiumoxide::Page;
use std::error::Error;
use tokio::time::{timeout, Duration};
use tracing::info;

pub async fn wait_for_element(
    browser: &mut Browser,
) -> Result<Element, Box<dyn std::error::Error>> {
    let page = browser
        .new_page("http://localhost:8000/delayed-clientside.html")
        .await?;
    return wait_for_element_visible(&page, "#come-find-me").await;
}

async fn wait_for_element_visible(
    page: &Page,
    selector: &str,
) -> Result<Element, Box<dyn std::error::Error>> {
    let timeout_duration = Duration::from_secs(32); // Adjust the timeout duration as needed

    let element_result = timeout(timeout_duration, async {
        loop {
            match page.find_element(selector).await {
                Ok(element) => return Ok(element),
                // Wait for a short interval before checking again
                Err(e) => tokio::time::sleep(Duration::from_millis(100)).await,
            }
        }
    })
    .await;
    match element_result {
        Ok(Ok(element)) => return Ok(element),
        Ok(Err(e)) => return Err(e),
        Err(_) => return Err("Element not found within the timeout duration".into()),
    }
}
