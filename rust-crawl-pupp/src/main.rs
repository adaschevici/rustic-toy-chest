use clap::Parser;
use futures::StreamExt;
use std::{thread, time};
use tracing::info;

use chromiumoxide::browser::{Browser, BrowserConfig};

mod first_project;
use crate::first_project::spoof_user_agent;

#[derive(Parser)]
#[command(
    name = "OxideCrawler",
    version = "0.1",
    author = "artur",
    about = "An example application using clap"
)]
struct Cli {
    #[arg(short, long = "first-project")]
    first_project: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();
    let args = Cli::parse();
    info!(args.first_project, "Starting up");

    let (mut browser, mut handler) = Browser::launch(
        BrowserConfig::builder()
            //.with_head()
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
    match args.first_project {
        true => {
            let user_agent = spoof_user_agent(&mut browser).await?;
            info!(user_agent, "User agent detected");
        }
        false => (),
    };

    browser.close().await?;
    handle.await?;
    Ok(())
}
