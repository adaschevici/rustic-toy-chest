use chromiumoxide::browser::Browser;
use chromiumoxide::element::Element;
use chromiumoxide::error::CdpError as Error;
use futures::stream::{self, StreamExt};

#[derive(Debug)]
pub struct Product {
    pub name: String,
    pub price: String,
}

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

pub async fn grab_list_of_elements_and_subelements_by_selector(
    browser: &mut Browser,
) -> Result<Vec<Result<Product, Error>>, Box<dyn std::error::Error>> {
    let page = browser
        .new_page("https://scrapingclub.com/exercise/list_infinite_scroll/")
        .await?;
    let elements_on_page = page.find_elements(".post").await?;
    let elements = stream::iter(elements_on_page)
        .then(|e| async move {
            let product_name = e.find_element("h4").await?.inner_text().await?.unwrap();
            let product_price = e.find_element("h5").await?.inner_text().await?.unwrap();
            Ok(Product {
                name: product_name,
                price: product_price,
            })
        })
        .collect::<Vec<_>>()
        .await;
    Ok(elements)
}
