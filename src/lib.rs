#![doc = include_str!("../README.md")]
#![no_std]

pub mod array;
pub mod error;
pub mod mapper;
pub mod single;

pub use array::Array;
pub use error::Error;
pub use mapper::Mapper;
pub use single::Single;

fn is_aligned<T>(phys_base: usize) -> bool {
    phys_base % core::mem::align_of::<T>() == 0
}
