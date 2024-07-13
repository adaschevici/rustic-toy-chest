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
    assert_eq!(struct_type, "Point");

    // we have two fields in our struct
    assert_eq!(ast.fields.len(), 2);

    // syn::Fields can be iterated over
    let mut iter = ast.fields.iter();

    let x_field = iter.next().unwrap();
    let x_field_name = x_field.ident.as_ref().unwrap().to_string();
    assert_eq!(x_field_name, "x");
}
