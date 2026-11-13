use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{
    DeriveInput, LitInt, Token,
    parse::{Parse, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
};

pub(crate) fn derive_named_impl(item: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(item as DeriveInput);

    let name = &ast.ident;

    let ret = quote! {
        impl crate::utils::named::Named for #name {
            fn name(&self) -> &str {
                &stringify!(#name)
            }
        }
    };

    TokenStream::from(ret)
}

enum IdItem {
    Single(u8),
    Range(u8, u8),
}

impl Parse for IdItem {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let start_lit: LitInt = input.parse()?;
        let start: u8 = start_lit.base10_parse()?;

        if input.peek(Token![-]) {
            input.parse::<Token![-]>()?;
            let end_lit: LitInt = input.parse()?;
            let end: u8 = end_lit.base10_parse()?;

            if start >= end {
                return Err(syn::Error::new_spanned(
                    end_lit,
                    "End must be greater than start",
                ));
            }

            Ok(IdItem::Range(start, end))
        } else {
            Ok(IdItem::Single(start))
        }
    }
}

pub(crate) fn dispatch_mapper_impl(input: TokenStream) -> TokenStream {
    let items = parse_macro_input!(input with Punctuated::<IdItem, Token![,]>::parse_terminated);

    let arms = items.into_iter().flat_map(|item| {
        let (s, e) = match item {
            IdItem::Range(s, e) => (s, e),
            IdItem::Single(v) => (v, v),
        };

        (s..=e).map(|num| {
            let mod_ident = format_ident!("mapper_{}", num);
            let struct_ident = format_ident!("Mapper{}", num);

            quote! {
                #num => boxed_mapper(crate::arch::mappers::#mod_ident::#struct_ident::new, header, buf),
            }
        })
    });

    TokenStream::from(quote! {
        match header.mapping_number() {
            #(#arms)*
            _ => anyhow::bail!("Invalid mapper"),
        }
    })
}
