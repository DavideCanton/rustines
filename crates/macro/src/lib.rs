mod mapper;
mod memento;

extern crate proc_macro;
extern crate quote;
extern crate syn;

use proc_macro::TokenStream;

#[proc_macro_derive(Named)]
pub fn derive_named(item: TokenStream) -> TokenStream {
    mapper::derive_named_impl(item)
}

#[proc_macro]
pub fn dispatch_mapper(input: TokenStream) -> TokenStream {
    mapper::dispatch_mapper_impl(input)
}

#[proc_macro_derive(Memento, attributes(memento))]
pub fn derive_memento(item: TokenStream) -> TokenStream {
    memento::derive_memento_impl(item)
}

#[proc_macro_derive(MementoBitfield)]
pub fn derive_memento_bitfield(item: TokenStream) -> TokenStream {
    memento::derive_memento_bitfield_impl(item)
}
