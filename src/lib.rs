#![no_std]

#[cfg(feature = "example")]
#[macro_use]
extern crate tagged_union_derive;

// TODO: uncomment this when the proc macro is fully functional
// #[cfg(feature = "example")]
// pub mod example;

use core::ops::Range;

pub trait TaggedUnion: Sized {
    type Target;

    /// Get the tag corresponding to this variant.
    fn tag(&self) -> u32;
    /// Get a FFI-safe version of this enum.
    fn as_tagged(&self) -> Self::Target;
    /// Try to convert back from a tagged union.
    unsafe fn from_tagged(tagged: &Self::Target) -> Result<Self, InvalidTag>;
}

pub struct InvalidTag {
    pub got: u32,
    pub possible_tags: Range<usize>,
}
