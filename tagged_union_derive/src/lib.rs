extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use syn::{Attribute, Body, DeriveInput, Generics, MetaItem, NestedMetaItem};


#[proc_macro_derive(TaggedUnion)]
pub fn tagged_union(input: TokenStream) -> TokenStream {
    let input = input.to_string();
    let definition = syn::parse_derive_input(&input).unwrap();

    if let Err(e) = apply_assertions(&definition) {
        panic!("{}", e);
    }

    quote!().parse().unwrap()
}

/// Ensure that various constraints are valid. At the moment these are:
/// 
/// - You can only `#[derive(TaggedUnion)]` on an enum
/// - The enum must be `Copy`
/// - It also can't be generic in any way (not even lifetimes)
fn apply_assertions(input: &DeriveInput) -> Result<(), String> {
    if let Body::Struct(_) = input.body {
        return Err("The TaggedUnion derive can only be used on enums.".to_string());
    }

    if is_generic(&input.generics) {
        return Err("You can't derive TaggedUnion on a generic type.".to_string());
    }

    Ok(())
}

fn is_generic(gen: &Generics) -> bool {
    !gen.lifetimes.is_empty() || !gen.ty_params.is_empty() || !gen.where_clause.predicates.is_empty()
}
