use workspace_macros::tea;

// #[route(path = "/hello", method = "GET")]
// fn hello() {
//     println!("Hello, world!");
// }
//
// #[route(path = "/goodbye", method = "POST")]
// fn goodbye() {
//     println!("Goodbye, world!");
// }
//
// pub async fn run_routing_to_functions() {
//     // This will call the hello function
//     #[call_fn(fn = "hello")]
//     fn my_function() {}
//
//     // This will call the goodbye function
//     #[call_fn(fn = "goodbye")]
//     fn another_function() {}
//
//     my_function();
//     another_function();
// }
//

#[tea(kind = "Green", hot, with(Lemon, Honey))]
struct Picard {
    rank: String,
}

pub async fn run_tea_making() {
    // This will call the hello function
    let pcrd = Picard {
        rank: "Captain".to_string(),
    };
}
