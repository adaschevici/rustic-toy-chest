use futures::future::BoxFuture;
use inquire::Select;
use tracing::info;

mod animal_behavior_macro;
mod procedural_macros;
mod routing_to_functions;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let functions: Vec<(&str, fn() -> BoxFuture<'static, ()>)> = vec![
        ("Run initial macro example", || {
            Box::pin(async {
                info!("Running initial macro example");
            })
        }),
        ("Run basic macro example", || {
            Box::pin(animal_behavior_macro::run_animal_behavior_macro())
        }),
        ("Run procedural macro example", || {
            Box::pin(procedural_macros::run_jsonify_macro())
        }),
        ("Run generic procedural macro example", || {
            Box::pin(procedural_macros::run_jsonify_generic_macro())
        }),
        ("Run routing to functions example", || {
            Box::pin(routing_to_functions::run_routing_to_functions())
        }),
        ("Run tea making facilities", || {
            Box::pin(routing_to_functions::run_tea_making_on_struct())
        }),
        ("Run tea making facilities on function", || {
            Box::pin(routing_to_functions::run_tea_making_on_function())
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
