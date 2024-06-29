use chromiumoxide::browser::Browser;

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
    std::thread::sleep(std::time::Duration::from_secs(30));
    let element_handle = page.find_element("come-find-me").await?;
    let product_name = element_handle.inner_text().await?.unwrap();
    println!("Product name: {}", product_name);
    Ok(())
}
