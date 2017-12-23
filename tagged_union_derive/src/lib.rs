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

    apply_assertions(&definition).unwrap();

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

    let is_copy_type = input.attrs.iter().find(|attr| is_copy_attr(attr)).is_some();

    if !is_copy_type {
        return Err(format!("Rust unions only (currently) work with Copy types, therefore {} also needs to be Copy.", input.ident));
    }

    if is_generic(&input.generics) {
        return Err("You can't derive TaggedUnion on a generic type.".to_string());
    }

    Ok(())
}

fn is_copy_attr(attr: &Attribute) -> bool {
    if attr.name() != "derive" {
        return false;
    }

    if let MetaItem::List(_, ref items) = attr.value {
        for item in items {
            if let NestedMetaItem::MetaItem(MetaItem::Word(ref ident)) = *item {
                if ident.to_string() == "Copy" {
                    return true;
                }
            }
        }
    }

    false
}

fn is_generic(gen: &Generics) -> bool {
    gen.lifetimes.is_empty() && gen.ty_params.is_empty() && gen.where_clause.predicates.is_empty()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_copy_types() {
        let inputs = vec![("#[derive(Copy)]", true), 
                          ("#[derive(Debug, Clone, PartialEq)]", false), 
                          ("#[macro_use]", false), 
                          ("#[rename = false]", false)];

        for (src, should_be) in inputs {
            let attr = syn::parse_outer_attr(src).unwrap();
            let got = is_copy_attr(&attr);

            assert_eq!(got, should_be, "{}", src);
        }
    }
}