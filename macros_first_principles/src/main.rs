use std::str::FromStr;

use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::ItemStruct;
use tracing::info;

fn main() {
    tracing_subscriber::fmt::init();
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

    let y_field = iter.next().unwrap();
    let y_field_name = y_field.ident.as_ref().unwrap().to_string();
    assert_eq!(y_field_name, "y");

    // first, build our function name: point_summation
    let function_name = format_ident!("{}_summation", struct_type.to_lowercase());

    // and our argument type. If we don't use the format ident macro, the function prototype
    // will be: pub fn point_summation (pt : "Point")
    let argument_type = format_ident!("{}", struct_type);

    // same for x and y
    let x = format_ident!("{}", x_field.ident.as_ref().unwrap());
    let y = format_ident!("{}", y_field.ident.as_ref().unwrap());

    // the quote!() macro is returning a new TokenStream. This TokenStream is returned to
    // the compiler in a "real" procedural macro
    let summation_fn = quote! {
        pub fn #function_name(pt: &#argument_type) -> u16 {
            pt.#x + pt.#y
        }
    };
    // output our function as Rust code
    info!("{}", summation_fn);
}
