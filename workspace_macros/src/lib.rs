extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use serde_json;
use syn::{parse_macro_input, Data, DeriveInput, ItemFn, Lit, LitStr, Meta, Path};

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
        Data::Struct(data_struct) => {
            let fields = data_struct.fields;
            let field_names = fields.iter().map(|field| &field.ident);
            let field_strings = field_names
                .clone()
                .map(|field| field.as_ref().unwrap().to_string());
            quote! {
                impl ToJsonGeneric for #name {
                    fn to_json(&self) -> String {
                        let mut map = ::std::collections::HashMap::new();
                        #(map.insert(#field_strings.to_string(), ::serde_json::to_value(&self.#field_names).unwrap());)*
                        ::serde_json::to_string(&map).unwrap()
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

// #[proc_macro_attribute]
// pub fn route(args: TokenStream, input: TokenStream) -> TokenStream {
//     // Parse the attribute arguments
//     let args = parse_macro_input!(args as AttributeArgs);
//
//     // Parse the function the attribute is applied to
//     let input = parse_macro_input!(input as ItemFn);
//
//     // Extract the route path and HTTP method from the attribute arguments
//     let mut route_path = String::new();
//     let mut method = String::new();
//
//     for arg in args {
//         match arg {
//             NestedMeta::Meta(Meta::NameValue(meta)) => {
//                 if meta.path.is_ident("path") {
//                     if let Lit::Str(lit) = meta {
//                         route_path = lit.value();
//                     }
//                 } else if meta.path.is_ident("method") {
//                     if let Lit::Str(lit) = meta {
//                         method = lit.value();
//                     }
//                 }
//             }
//             _ => {}
//         }
//     }
//
//     // Extract the function name
//     let fn_name = &input.sig.ident;
//
//     // Generate the routing code
//     let expanded = quote! {
//         #input
//
//         // Register the route
//         fn register_routes() {
//             let route_path = #route_path;
//             let method = #method.to_string();
//
//             // This is where you would integrate with your routing library
//             // For example:
//             // router.add_route(method, route_path, #fn_name);
//             println!("Registered route: {} {}", method, route_path);
//         }
//     };
//
//     TokenStream::from(expanded)
// }

// #[proc_macro_attribute]
// pub fn call_fn(args: TokenStream, input: TokenStream) -> TokenStream {
//     // Parse the attribute arguments
//     let args = parse_macro_input!(args as AttributeArgs);
//
//     // Parse the function the attribute is applied to
//     let input = parse_macro_input!(input as ItemFn);
//
//     // Extract the function name from the attribute arguments
//     let mut fn_to_call = None;
//
//     for arg in args {
//         if let NestedMeta::Meta(Meta::NameValue(meta_name_value)) = arg {
//             if meta_name_value.path.is_ident("fn") {
//                 fn_to_call = Some(meta_name_value.value()?.parse()?);
//             }
//         }
//     }
//
//     // Match the function name to the appropriate function call
//     let expanded = match Some(quote!(#fn_to_call).to_string().as_str()) {
//         Some("hello") => quote! {
//             #input
//             fn call_fn() {
//                 hello();
//             }
//         },
//         Some("goodbye") => quote! {
//             #input
//             fn call_fn() {
//                 goodbye();
//             }
//         },
//         _ => quote! {
//             #input
//             compile_error!("Unknown function specified in the attribute");
//         },
//     };
//
//     TokenStream::from(expanded)
// }

#[proc_macro_attribute]
pub fn tea(args: TokenStream, input: TokenStream) -> TokenStream {
    let mut kind: Option<LitStr> = None;
    let mut hot: bool = false;
    let mut with: Vec<Path> = Vec::new();
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let tea_parser = syn::meta::parser(|meta| {
        if meta.path.is_ident("kind") {
            kind = Some(meta.value()?.parse()?);
            Ok(())
        } else if meta.path.is_ident("hot") {
            hot = true;
            Ok(())
        } else if meta.path.is_ident("with") {
            meta.parse_nested_meta(|meta| {
                with.push(meta.path);
                Ok(())
            })
        } else {
            Err(meta.error("unsupported tea property"))
        }
    });

    parse_macro_input!(args with tea_parser);
    eprintln!("hot={hot}");

    let output = quote! {
        #input
        // impl #name() {
        //     println!("Tea kind: {}", #kind);
        // }
        fn #name() {
            format!("Tea kind: {}", #kind);
        }
    };

    TokenStream::from(output)
}
