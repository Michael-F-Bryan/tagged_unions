extern crate proc_macro;

use proc_macro::TokenStream;


#[proc_macro_derive(TaggedUnion)]
pub fn tagged_union(input: TokenStream) -> TokenStream {
    "".parse().unwrap()
}