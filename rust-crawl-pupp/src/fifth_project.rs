use chromiumoxide::browser::Browser;
use chromiumoxide::cdp::js_protocol::runtime::EvaluateParams;

pub async fn scroll_to_bottom(browser: &mut Browser) -> Result<(), Box<dyn std::error::Error>> {
    let js_script = r#"
        async () => {
          await new Promise((resolve, reject) => {
            var totalHeight = 0;
            var distance = 300; // should be less than or equal to window.innerHeight
            var timer = setInterval(() => {
              var scrollHeight = document.body.scrollHeight;
              window.scrollBy(0, distance);
              totalHeight += distance;

              if (totalHeight >= scrollHeight) {
                clearInterval(timer);
                resolve();
              }
            }, 500);
          });
      }
    "#;
    let page = browser
        .new_page("https://scrapingclub.com/exercise/list_infinite_scroll/")
        .await?;
    page.evaluate_function(js_script).await?;
    std::thread::sleep(std::time::Duration::from_secs(50));
    Ok(())
}
