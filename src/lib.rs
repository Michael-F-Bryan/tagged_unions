#![no_std]
use core::ops::Range;


pub trait TaggedUnion: Sized {
    type Target;

    /// Get the tag corresponding to this variant.
    fn tag(&self) -> usize;
    /// Get a FFI-safe version of this enum.
    fn as_tagged(&self) -> Self::Target;
    /// Try to convert back from a tagged union.
    fn from_tagged(tagged: &Self::Target) -> Result<Self, InvalidTag>;
}

pub struct InvalidTag {
    pub got: usize,
    pub possible_tags: Range<usize>,
}