use chromiumoxide::browser::Browser;
// use chromiumoxide::cdp::browser_protocol::page::Frame;
use chromiumoxide::handler::frame::Frame;
use tracing::info;

pub async fn get_page_frames(browser: &mut Browser) -> Result<(), Box<dyn std::error::Error>> {
    let page = browser
        .new_page("http://localhost:8000/iframe.html")
        .await?;
    let frames = page.frames().await?;
    for frame in frames {
        println!("Frame: {:?}", frame);
    }
    Ok(())
}

pub async fn get_nested_iframe_element(
    browser: &mut Browser,
) -> Result<(), Box<dyn std::error::Error>> {
    let page = browser
        .new_page("http://localhost:8000/iframe.html")
        .await?;
    let iframe = page.find_element("iframe").await?;
    let child_frame = iframe.find_element("#come-find-me-later").await?;
    // let frames = page.frames().await?;
    // for frame in frames {
    //     let current_frame = Frame::new(frame.clone());
    //     info!("Current frame: {:?}", current_frame.main_world());
    //     // if current_frame.is_main() {
    //     //     info!("Main frame: {:?}", frame);
    //     //     continue;
    //     // }
    // }
    // tokio::time::sleep(std::time::Duration::from_secs(30)).await;
    // page.wait_for_navigation_response().await?;
    // let element_handle = page.find_element("#come-find-me-later").await?;
    // let product_name = element_handle.inner_text().await?.unwrap();
    // println!("Product name: {}", product_name);
    Ok(())
}
