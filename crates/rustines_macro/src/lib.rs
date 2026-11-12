use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{DeriveInput, LitInt, Token, parse_macro_input, punctuated::Punctuated};
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

#[proc_macro]
pub fn dispatch_mapper(input: TokenStream) -> TokenStream {
    let ids = parse_macro_input!(input with Punctuated::<LitInt, Token![,]>::parse_terminated);

    let arms = ids.iter().map(|lit| {
        let num = lit.base10_parse::<u16>().unwrap();
        let mod_ident = format_ident!("mapper_{}", num);
        let struct_ident = format_ident!("Mapper{}", num);

        quote! {
            #lit => boxed_mapper(crate::arch::mappers::#mod_ident::#struct_ident::new, header, buf),
        }
    });

    TokenStream::from(quote! {
        match header.mapping_number() {
            #(#arms)*
            _ => anyhow::bail!("Invalid mapper"),
        }
    })
}
