use futures::future::BoxFuture;
use inquire::Select;
use std::error::Error;
use tracing::info;

mod custom_future;
mod error_handling;
mod perform_basic_request;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let functions: Vec<(&str, fn() -> BoxFuture<'static, ()>)> = vec![
        ("Run simple request", || {
            Box::pin(perform_basic_request::perform_basic_request())
        }),
        ("Run custom future", || {
            Box::pin(custom_future::use_custom_future())
        }),
        ("Run error handling", || {
            Box::pin(error_handling::read_file_with_error_handling())
        }),
    ];
    let function_names: Vec<&str> = functions.iter().map(|(name, _)| *name).collect();

    // Prompt the user to select a function
    let selected_function = Select::new("Choose a function to execute:", function_names.clone())
        .with_starting_cursor(function_names.len() - 1)
        .prompt()
        .expect("Failed to read input");

    // Find and execute the corresponding function
    for (name, function) in functions {
        if name == selected_function {
            function().await;
        }
    }
}
