use clap::{Parser, Subcommand};
use futures::StreamExt;
use std::{thread, time};
use tracing::info;

use chromiumoxide::browser::{Browser, BrowserConfig};

mod first_project;
mod second_project;
use crate::first_project::spoof_user_agent;
use crate::second_project::grab_root_content;

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
        _ => {
            println!("{:#?}", args.command);
        }
    }
    browser.close().await?;
    handle.await?;
    Ok(())
}
