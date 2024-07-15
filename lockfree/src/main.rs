use futures::future::BoxFuture;
use inquire::Select;
use tracing::info;

mod non_locker;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let functions: Vec<(&str, fn() -> BoxFuture<'static, ()>)> = vec![
        ("Run initial macro example", || {
            Box::pin(async {
                info!("Running initial noop option");
            })
        }),
        ("Run non-locker-stack example", || {
            Box::pin(non_locker::run_nonlocker_stack_ops())
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