extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput};

mod to_json;
use to_json::{ToJson, ToJsonGeneric};

#[proc_macro_derive(ToJson)]
pub fn to_json_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let expanded = quote! {
        impl ToJson for #name {
            fn to_json(&self) -> String {
                format!(
                    r#"{{ "name": "{}", "age": {}, "is_alive": {} }}"#,
                    self.name, self.age, self.is_alive
                )
            }
        }
    };
    TokenStream::from(expanded)
}

#[proc_macro_derive(ToJsonGeneric)]
pub fn to_json_generic_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let expanded = match input.data {
        Data::Struct(_) => {
            quote! {
                impl ToJsonGeneric for #name {
                    fn to_json(&self) -> String {
                        format!(
                            r#"{{ "name": "{}", "age": {}, "is_alive": {} }}"#,
                            self.name, self.age, self.is_alive
                        )
                    }
                }
            }
        }
        _ => {
            quote! {
                impl ToJsonGeneric for #name {
                    fn to_json(&self) -> String {
                        String::from("Not a struct")
                    }
                }
            }
        }
    };
    TokenStream::from(expanded)
}
