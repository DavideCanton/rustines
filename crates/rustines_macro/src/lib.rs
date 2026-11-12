use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{
    Data, DeriveInput, Fields, Index, LitInt, Member, Meta, Token, parse_macro_input,
    punctuated::Punctuated,
};
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

// TODO support enums?
#[proc_macro_derive(Memento, attributes(memento))]
pub fn derive_memento(item: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(item as DeriveInput);

    let name = &ast.ident;

    let fields: Vec<_> = match &ast.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => fields
                .named
                .iter()
                .map(|f| (Member::Named(f.ident.clone().unwrap()), f))
                .collect(),
            Fields::Unit => Vec::new(),
            Fields::Unnamed(fields) => fields
                .unnamed
                .iter()
                .enumerate()
                .map(|(i, f)| (Member::Unnamed(Index::from(i)), f))
                .collect(),
        },
        _ => panic!("Only structs are supported"),
    };

    let fields_empty = fields.is_empty();

    let mut filtered = vec![];

    for (n, f) in fields {
        let attr = f.attrs.iter().find(|a| a.path().is_ident("memento"));
        if let Some(attr) = attr {
            let mut is_fill: bool = false;

            match &attr.meta {
                Meta::Path(_) => {
                    is_fill = false;
                }
                Meta::List(_) => {
                    attr.parse_nested_meta(|meta| {
                        if meta.path.is_ident("fill") {
                            is_fill = true;
                            Ok(())
                        } else {
                            Err(meta.error("Invalid argument"))
                        }
                    })
                    .unwrap();
                }
                Meta::NameValue(_) => {
                    panic!("Invalid format");
                }
            }

            filtered.push((n, is_fill));
        }
    }

    if !fields_empty && filtered.is_empty() {
        panic!("No fields with #[memento]");
    }

    let mut ts = generate_memento_readable(name, &filtered);
    ts.extend(generate_memento_writeable(name, &filtered));
    ts
}

#[proc_macro_derive(MementoBitfield)]
pub fn derive_memento_bitfield(item: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(item as DeriveInput);

    let name = &ast.ident;

    let fields: Vec<_> = match &ast.data {
        Data::Struct(data) => match &data.fields {
            Fields::Unnamed(fields) => fields
                .unnamed
                .iter()
                .enumerate()
                .map(|(i, _)| Member::Unnamed(Index::from(i)))
                .collect(),
            _ => panic!("Only unnamed structs with one field are supported"),
        },
        _ => panic!("Only structs are supported"),
    };

    if fields.len() != 1 {
        panic!("Only a single field is supported");
    }

    let ident = fields.into_iter().next().unwrap();

    let slice = &[(ident, false)];

    let mut ts = generate_memento_writeable(name, slice);
    ts.extend(generate_memento_readable(name, slice));
    ts
}

fn generate_memento_writeable(name: &syn::Ident, fields: &[(Member, bool)]) -> TokenStream {
    let mut exprs = Vec::new();

    for (fname, _) in fields {
        exprs.push(quote! {
            self.#fname.write_memento(writer)?;
        });
    }

    let ret = quote! {
        impl MementoWriteable for #name {
            fn write_memento(&self, writer: &mut dyn MementoWriter) -> MementoResult {
                #(#exprs)*
                Ok(())
            }
        }
    };

    TokenStream::from(ret)
}

fn generate_memento_readable(name: &syn::Ident, fields: &[(Member, bool)]) -> TokenStream {
    let mut exprs = Vec::new();

    for (fname, is_fill) in fields {
        let q = if *is_fill {
            quote! {
                self.#fname.read_and_fill(reader)?;
            }
        } else {
            quote! {
                self.#fname = reader.read_memento()?;
            }
        };
        exprs.push(q);
    }

    let ret = quote! {
        impl MementoReadAndFill for #name {
            fn read_and_fill(&mut self, reader: &mut MementoReader) -> MementoResult {
                #(#exprs)*
                Ok(())
            }
        }
    };

    TokenStream::from(ret)
}
