#[macro_use]
extern crate failure;
extern crate proc_macro;
#[macro_use]
extern crate quote;
extern crate syn;

mod codegen;

use proc_macro::TokenStream;

#[proc_macro_derive(TaggedUnion)]
pub fn tagged_union(input: TokenStream) -> TokenStream {
    let input = input.to_string();
    let definition = syn::parse_derive_input(&input).unwrap();

    match codegen::expand(&definition) {
        Ok(tokens) => tokens.parse().unwrap(),
        Err(e) => panic!("{}", e),
    }
}
