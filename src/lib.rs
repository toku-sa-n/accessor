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

pub use memoffset;

fn is_aligned<T>(phys_base: usize) -> bool {
    phys_base % core::mem::align_of::<T>() == 0
}
