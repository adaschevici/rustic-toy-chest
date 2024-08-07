use futures::future::BoxFuture;
use inquire::Select;
use tracing::info;

mod matrix_mul;
mod mrg_sort;
mod parsum;
mod rayon_custom_it;
mod track_racers;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let functions: Vec<(&str, fn() -> BoxFuture<'static, ()>)> = vec![
        ("Run noop operation", || {
            Box::pin(async {
                info!("Running initial noop option");
            })
        }),
        (
            "Run rayon race",
            || Box::pin(track_racers::run_race_event()),
        ),
        ("Run rayon parsum", || Box::pin(parsum::run_parsum())),
        ("Run rayon parsum map filter", || {
            Box::pin(parsum::run_parsum_map_filter())
        }),
        ("Run rayon parsort", || Box::pin(parsum::run_parsort())),
        ("Run rayon custom iterator", || {
            Box::pin(rayon_custom_it::iterate_in_chunks())
        }),
        ("Run rayon matrix multiplication", || {
            Box::pin(matrix_mul::run_matrix_mul())
        }),
        (
            "Run rayon merge sort",
            || Box::pin(mrg_sort::run_mrg_sort()),
        ),
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
