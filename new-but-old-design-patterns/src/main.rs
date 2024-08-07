use futures::future::BoxFuture;
use inquire::Select;
use tracing::info;

mod adapter;
mod command;
mod decorator;
mod facade;
mod factory;
mod iterator;
mod observer;
mod state_pattern;
mod strategy;

use adapter::run_adapter;
use observer::run_observer;
use state_pattern::run_state_pattern;
use strategy::run_strategy;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    // Define the list of functions
    info!("Defining functions");

    let functions: Vec<(&str, fn() -> BoxFuture<'static, ()>)> = vec![
        (
            "Run state pattern example",
            || Box::pin(run_state_pattern()),
        ),
        ("Run observer pattern example", || Box::pin(run_observer())),
        ("Run strategy pattern example", || Box::pin(run_strategy())),
        ("Run iterator pattern example", || {
            Box::pin(iterator::run_iterator())
        }),
        ("Run command pattern example", || {
            Box::pin(command::run_command())
        }),
        ("Run decorator pattern example", || {
            Box::pin(decorator::run_decorator())
        }),
        ("Run factory pattern example", || {
            Box::pin(factory::run_factory())
        }),
        ("Run adapter pattern example", || Box::pin(run_adapter())),
        ("Run facade pattern example", || {
            Box::pin(facade::run_facade())
        }),
    ];

    // Create a vector of function names
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
