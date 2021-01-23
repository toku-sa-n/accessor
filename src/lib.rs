//! Accessors to access physical memory.
//!
//! This crate is intended to access memory-mapped I/O. Reading and writing are done volatilely.
//!
//! This crate is `#[no_std]` compatible.
//!
//! ```no_run
//! use accessor::mapper::Mapper;
//!
//! struct M;
//! impl Mapper for M {
//!     unsafe fn map(&mut self, phys_start: usize, bytes: usize) -> usize {
//!         unimplemented!()
//!     }
//!
//!     fn unmap(&mut self, phys_start: usize, bytes: usize) {
//!         unimplemented!()
//!     }
//! }
//!
//! // Create an accessor to an i32 value at the physical address 0x1000.
//! let mut a = unsafe {
//!     accessor::Single::<i32, M>::new(0x1000, M).expect("Failed to create an accessor.")
//! };
//!
//! // Read a value.
//! a.read();
//!
//! // Write a value.
//! a.write(3);
//!
//! // Create an accessor to an array at the physical address 0x2000 of the type i32 that has 5 elements.
//! let mut arr = unsafe {
//!     accessor::Array::<i32, M>::new(0x2000, 5, M).expect("Failed to create an accessor.")
//! };
//!
//! // Read the 2nd element.
//! arr.read_at(2);
//!
//! // Write 42 as the 0th element.
//! arr.write_at(0, 42);
//! ```

#![no_std]

pub mod array;
pub mod error;
pub mod mapper;
pub mod single;

pub use array::Array;
pub use single::Single;

fn is_aligned<T>(phys_base: usize) -> bool {
    phys_base % core::mem::align_of::<T>() == 0
}
