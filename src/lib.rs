#![doc = include_str!("../README.md")]
#![no_std]

pub mod array;
pub mod error;
pub mod mapper;
pub mod marker;
pub mod single;

#[allow(deprecated)]
pub use {array::Array, single::Single};

pub use {error::Error, mapper::Mapper};

/// A derive macro which converts a field struct type into a struct of accessors with same field names.
/// For a field struct type `T`, add `#[derive(BoundedStructuralOf)]` before the struct definition to derive
/// accessor struct type `SingleBoundedStructuralOfT<'a, M, A>` and `ArrayBoundedStructuralOfT<'a, M, A>`.
/// 
/// See [`single::BoundedStructural`] and [`array::BoundedStructural`]  for details.
pub use accessor_macros::BoundedStructuralOf;

#[doc(hidden)]
pub use memoffset;

fn is_aligned<T>(phys_base: usize) -> bool {
    phys_base % core::mem::align_of::<T>() == 0
}
