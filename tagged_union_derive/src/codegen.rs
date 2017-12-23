use std::collections::HashMap;

use failure::Error;
use syn::{self, Attribute, Body, DeriveInput, Generics, MetaItem, NestedMetaItem, Ident, Ty, Variant, VariantData};
use quote::Tokens;

type TypeMap = HashMap<Ty, Vec<Ident>>;

/// Analyse the derive input
/// 
/// For some enum, `Foo`, this will:
/// 
/// - Generate a list of tags
/// - Create a `TaggedFoo` struct for `Foo`
/// - Create the `FooKind` union
/// - Implement `TaggedUnion` for `Foo`.
pub fn expand(input: &DeriveInput) -> Result<Tokens, Error> {
    let variants = match input.body {
        Body::Enum(ref var) => var,
        Body::Struct(_) => bail!("The TaggedUnion derive can only be used on enums."),
    };

    if is_generic(&input.generics) {
        bail!("You can't derive TaggedUnion on a generic type.");
    }

    let tags = generate_tags(input.ident.as_ref(), variants);
    let typemap = typemap_for(&variants)?;

    unimplemented!()
}

/// Generate a list of all the types the `FooKind` union will need to contain.
/// Because you can get more than one variant containing the same underlying 
/// type we use a `TypeMap` to preserve the relations.
fn typemap_for(variants: &[Variant]) -> Result<TypeMap, Error> {
    let mut map = TypeMap::new();
    let unit = syn::parse_type("()").unwrap();

    for variant in variants {
        match variant.data {
            VariantData::Struct(_) => bail!("Struct variants aren't supported."),
            VariantData::Tuple(ref fields) =>  {
                if fields.len() > 1 {
                    bail!("Tuple variants with more than one element aren't supported.");
                }

                map.entry(fields[0].ty.clone()).or_insert_with(Default::default).push(variant.ident.clone());
            }
            VariantData::Unit => {
                map.entry(unit.clone()).or_insert_with(Default::default).push(variant.ident.clone());
            }
        }
    }

    Ok(map)
}

fn is_generic(gen: &Generics) -> bool {
    !gen.lifetimes.is_empty() || !gen.ty_params.is_empty() || !gen.where_clause.predicates.is_empty()
}

fn generate_tags(enum_name: &str, variants: &[Variant]) -> Vec<Tag> {
    let prefix = enum_name.to_uppercase();

    variants.iter()
        .map(|var| var.ident.as_ref())
        .map(|name| format!("{}_{}", prefix, name.to_uppercase()))
        .enumerate()
        .map(|(i, name)| Tag::new(name, i as u32))
        .collect()
}

#[derive(Debug, Clone, PartialEq)]
struct Tag {
    name: String,
    number: u32,
}

impl Tag {
    pub fn new<S: Into<String>>(name: S, number: u32) -> Tag {
        Tag { 
            name: name.into(),
            number: number,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse_enum(src: &str) -> (DeriveInput, Vec<Variant>) {
        let parsed = syn::parse_derive_input(src).unwrap();

        let variants = match parsed.body {
            Body::Enum(ref var) => var.clone(),
            _ => unreachable!(),
        };

        (parsed, variants)
    }

    #[test]
    fn tag_for_variant() {
        let src = "enum Foo { Halt, Move(usize), Wait { secs: i64 }, }";
        let should_be = vec![
            Tag::new("FOO_HALT", 0),
            Tag::new("FOO_MOVE", 1),
            Tag::new("FOO_WAIT", 2),
        ];

        let (parsed, variants) = parse_enum(src);

        let got = generate_tags(parsed.ident.as_ref(), &variants);

        assert_eq!(got, should_be);
    }

    #[test]
    fn generate_typemap() {
        let src = "enum Foo { Halt, Move(usize), Wait(Bar), }";
        let should_be = vec![
            ("()", "Halt"),
            ("usize", "Move"),
            ("Bar", "Wait"),
        ];

        let should_be: TypeMap = should_be.into_iter().map(|(ty, name)| (syn::parse_type(ty).unwrap(), 
        vec![syn::parse_ident(name).unwrap()])).collect();

        let (_, variants) = parse_enum(src);

        let got = typemap_for(&variants).unwrap();

        assert_eq!(got, should_be);
    }
}