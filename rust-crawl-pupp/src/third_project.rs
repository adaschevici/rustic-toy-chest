use chromiumoxide::browser::Browser;
use chromiumoxide::element::Element;
use futures::stream::{self, StreamExt};

pub async fn grab_list_of_elements_by_selector(
    browser: &mut Browser,
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let page = browser
        .new_page("https://scrapingclub.com/exercise/list_infinite_scroll/")
        .await?;
    let elements_on_page = page.find_elements(".post").await?;
    let elements = stream::iter(elements_on_page)
        .then(|e| async move {
            let el_text = e.inner_text().await.ok();
            match el_text {
                Some(text) => text,
                None => None,
            }
        })
        .filter_map(|x| async { x })
        .collect::<Vec<_>>()
        .await;
    Ok(elements)
}
