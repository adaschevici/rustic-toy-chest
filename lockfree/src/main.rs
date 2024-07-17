use futures::future::BoxFuture;
use inquire::Select;
use tracing::info;

mod non_locking_queue;
mod non_locking_stack;
mod threaded_samples;

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
            Box::pin(non_locking_stack::run_non_locking_stack_ops())
        }),
        ("Run non-locker-queue example", || {
            Box::pin(non_locking_queue::run_non_locking_queue_ops())
        }),
        ("Run threaded example", || {
            Box::pin(threaded_samples::run_threaded_ops())
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
