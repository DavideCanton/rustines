use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Field, Fields, Ident, Index, Member, Meta, parse_macro_input};

// TODO support enums?

struct FieldMember<'f> {
    member: Member,
    field: &'f Field,
}

impl<'f> FieldMember<'f> {
    fn named(field: &'f Field) -> Self {
        Self {
            member: Member::Named(field.ident.as_ref().unwrap().clone()),
            field,
        }
    }

    fn unnamed(field: &'f Field, index: usize) -> Self {
        Self {
            member: Member::Unnamed(Index::from(index)),
            field,
        }
    }

    fn into_member_fill(self, is_fill: bool) -> MemberFill {
        MemberFill::new(self.member, is_fill)
    }
}

struct MemberFill {
    member: Member,
    is_fill: bool,
}

impl MemberFill {
    fn new(member: Member, is_fill: bool) -> Self {
        MemberFill { member, is_fill }
    }
}

pub(crate) fn derive_memento_impl(item: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(item as DeriveInput);

    let struct_name = &ast.ident;

    let fields: Vec<_> = if let Data::Struct(data) = &ast.data {
        match &data.fields {
            Fields::Named(fields) => fields.named.iter().map(FieldMember::named).collect(),
            Fields::Unnamed(fields) => fields
                .unnamed
                .iter()
                .enumerate()
                .map(|(i, f)| FieldMember::unnamed(f, i))
                .collect(),
            Fields::Unit => Vec::new(),
        }
    } else {
        panic!("Only structs are supported")
    };

    let fields_empty = fields.is_empty();

    let mut filtered = vec![];

    for field_member in fields {
        let attr = field_member
            .field
            .attrs
            .iter()
            .find(|a| a.path().is_ident("memento"));

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

            filtered.push(field_member.into_member_fill(is_fill));
        }
    }

    if !fields_empty && filtered.is_empty() {
        panic!("No fields with #[memento]");
    }

    combine_token_stream(
        generate_memento_writeable(struct_name, &filtered),
        generate_memento_readable(struct_name, &filtered),
    )
}

pub(crate) fn derive_memento_bitfield_impl(item: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(item as DeriveInput);

    let name = &ast.ident;

    let fields: Vec<_> = if let Data::Struct(data) = &ast.data {
        if let Fields::Unnamed(fields) = &data.fields {
            fields
                .unnamed
                .iter()
                .enumerate()
                .map(|(i, _)| Member::Unnamed(Index::from(i)))
                .collect()
        } else {
            panic!("Only unnamed structs with one field are supported")
        }
    } else {
        panic!("Only structs are supported")
    };

    if fields.len() != 1 {
        panic!("Only a single field is supported");
    }

    let member = fields.into_iter().next().unwrap();

    let slice = &[MemberFill::new(member, false)];

    combine_token_stream(
        generate_memento_writeable(name, slice),
        generate_memento_readable(name, slice),
    )
}

fn combine_token_stream(ts1: TokenStream, ts2: TokenStream) -> TokenStream {
    ts1.into_iter().chain(ts2).collect()
}

fn generate_memento_writeable(struct_name: &Ident, fields: &[MemberFill]) -> TokenStream {
    let mut statements = Vec::new();

    for field in fields {
        let fname = &field.member;
        statements.push(quote! {
            self.#fname.write_memento(writer)?;
        });
    }

    TokenStream::from(quote! {
        impl MementoWriteable for #struct_name {
            fn write_memento(&self, writer: &mut dyn MementoWriter) -> MementoResult {
                #(#statements)*
                Ok(())
            }
        }
    })
}

fn generate_memento_readable(struct_name: &Ident, fields: &[MemberFill]) -> TokenStream {
    let mut statements = Vec::new();

    for field in fields {
        let fname = &field.member;
        let statement = if field.is_fill {
            quote! {
                self.#fname.read_and_fill(reader)?;
            }
        } else {
            quote! {
                self.#fname = reader.read_memento()?;
            }
        };
        statements.push(statement);
    }

    TokenStream::from(quote! {
        impl MementoReadAndFill for #struct_name {
            fn read_and_fill(&mut self, reader: &mut MementoReader) -> MementoResult {
                #(#statements)*
                Ok(())
            }
        }
    })
}
