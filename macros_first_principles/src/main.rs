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
}
