use workspace_macros::{tea_over_fn, tea_over_struct};

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

// #[tea_over_struct(kind = "Green", hot, with(Lemon, Honey))]
// struct Picard {
//     rank: String,
// }
pub async fn run_tea_making_on_struct() {
    // This will call the hello function
    // let pcrd = Picard {
    //     rank: "Captain".to_string(),
    // };
    // pcrd.describe_tea();
}

#[tea_over_fn(kind = "Green", hot, with(Lemon, Honey))]
struct PicardFn {
    rank: String,
}

pub async fn run_tea_making_on_function() {
    // This will call the hello function

    let pcrd = PicardFn {
        rank: "Captain".to_string(),
    };
}
