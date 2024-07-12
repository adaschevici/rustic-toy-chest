use std::str::FromStr;

use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::ItemStruct;

fn main() {
    let s = r#"
        struct Point {
            x: i32,
            y: i32,
        }"#;
    let tokens = TokenStream::from_str(s).unwrap();

    // build the AST: note the syn::parse2() method rather than the syn::parse() one
    // which is meant for "real" procedural macros
    let ast: ItemStruct = syn::parse2(tokens).unwrap();

    // save our struct type for later use
    let struct_type = ast.ident.to_string();
    println!("struct type: {}", struct_type);
}
