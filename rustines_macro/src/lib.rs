use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};
extern crate proc_macro;
extern crate quote;
extern crate syn;

#[proc_macro_derive(Named)]
pub fn derive_named(item: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(item as DeriveInput);

    let name = &ast.ident;

    let ret = quote! {
        impl Named for #name {
            fn name(&self) -> &str {
                &stringify!(#name)
            }
        }
    };

    TokenStream::from(ret)
}
