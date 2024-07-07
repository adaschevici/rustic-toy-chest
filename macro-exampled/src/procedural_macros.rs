use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(ToJson)]
pub fn to_json_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
}

#[derive(ToJson)]
struct Person {
    name: String,
    age: u8,
    is_alive: bool,
}

pub async fn run_jsonify_macro() {
    let person = Person {
        name: "Alice".to_string(),
        age: 30,
        is_alive: true,
    };
    let json = person.to_json();
    println!("{}", json);
}
