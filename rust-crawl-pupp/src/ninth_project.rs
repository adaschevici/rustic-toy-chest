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
