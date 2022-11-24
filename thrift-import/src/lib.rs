use proc_macro::TokenStream;

use std::{env, path::PathBuf};

use quote::quote;
use syn::parse_macro_input;

#[proc_macro]
pub fn import(input: TokenStream) -> TokenStream {
    let name = parse_macro_input!(input as syn::Ident);

    let bindings_dir = env::var("THRIFT_OUT_DIR").expect("Cannot locate generated bindings");
    let bindings = PathBuf::from(bindings_dir).join(format!("{}.rs", name));
    let module_path = bindings.to_string_lossy();

    let tokens = quote! {
        #[path=#module_path]
        pub mod #name;
    };

    tokens.into()
}
