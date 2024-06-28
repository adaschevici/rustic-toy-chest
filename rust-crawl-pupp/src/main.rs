use clap::{Parser, Subcommand};
use futures::StreamExt;
use std::{thread, time};
use tracing::info;

use chromiumoxide::browser::{Browser, BrowserConfig};

mod fifth_project;
mod first_project;
mod ninth_project;
mod second_project;
mod seventh_project;
mod sixth_project;
mod third_project;

use crate::first_project::spoof_user_agent;
use crate::second_project::grab_root_content;
use crate::third_project::{
    grab_list_of_elements_and_subelements_by_selector, grab_list_of_elements_by_selector,
};
use fifth_project::scroll_to_bottom;
use ninth_project::get_page_frames;
use seventh_project::{capture_full_page_screenshot, capture_selector_screenshot};
use sixth_project::wait_for_element;

#[derive(Parser)]
#[command(
    name = "OxideCrawler",
    version = "0.1",
    author = "artur",
    about = "An example application using clap"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    FirstProject {},
    SecondProject {},
    ThirdProject {},
    FourthProject {},
    FifthProject {},
    SixthProject {},
    SeventhProject {},
    EighthProject {},
    NinthProject {},
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();
    let args = Cli::parse();

    let (mut browser, mut handler) = Browser::launch(
        BrowserConfig::builder()
            .with_head()
            .no_sandbox()
            .viewport(None)
            .window_size(1400, 1600)
            .build()?,
    )
    .await?;

    let handle = tokio::task::spawn(async move {
        loop {
            match handler.next().await {
                Some(h) => match h {
                    Ok(_) => continue,
                    Err(_) => break,
                },
                None => break,
            }
        }
    });
    match &args.command {
        Commands::FirstProject {} => {
            let user_agent = spoof_user_agent(&mut browser).await?;
            info!(user_agent, "User agent detected");
        }
        Commands::SecondProject {} => {
            let content = grab_root_content(&mut browser).await?;
            info!("{} {}", content.len(), "Length of root body content");
        }
        Commands::ThirdProject {} => {
            let elements = grab_list_of_elements_by_selector(&mut browser).await?;
            info!("{} {}", elements.len(), "Number of elements found");
            info!("{:?}", elements);
        }
        Commands::FourthProject {} => {
            let elements = grab_list_of_elements_and_subelements_by_selector(&mut browser).await?;
            info!("{} {}", elements.len(), "Number of elements found");
            info!("{:?}", elements);
        }
        Commands::FifthProject {} => {
            let _ = scroll_to_bottom(&mut browser).await?;
            let elements = grab_list_of_elements_by_selector(&mut browser).await?;
            info!("{} {}", elements.len(), "Number of elements found");
            info!("{:?}", elements);
        }
        Commands::SixthProject {} => {
            let outcome = wait_for_element(&mut browser).await;
            match outcome {
                Ok(_) => {
                    info!("Element found");
                }
                Err(e) => {
                    info!("{:?}", e);
                }
            }
        }
        Commands::SeventhProject {} => {
            let _ = capture_full_page_screenshot(&mut browser).await?;
        }
        Commands::EighthProject {} => {
            let _ = capture_selector_screenshot(&mut browser, ".post:nth-child(1)").await?;
        }
        Commands::NinthProject {} => {
            let frames = get_page_frames(&mut browser).await?;
            info!("{:?}", frames);
        }
        _ => {
            println!("{:#?}", args.command);
        }
    }
    browser.close().await?;
    browser.wait().await?;
    handle.await?;
    Ok(())
}
