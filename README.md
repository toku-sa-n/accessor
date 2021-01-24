# accessor

[![GitHub Actions](https://github.com/toku-sa-n/accessor/workflows/Rust/badge.svg)](https://github.com/toku-sa-n/accessor/actions)
[![Crates.io](https://img.shields.io/crates/v/accessor)](https://crates.io/crates/accessor)
![Crates.io](https://img.shields.io/crates/l/accessor)
[![docs.rs](https://docs.rs/accessor/badge.svg)](https://docs.rs/accessor)

Accessors to access physical memory.

This crate is intended to access memory-mapped I/O. Reading and writing are done volatilely.

The accessed type must implement [`Copy`] because reading and writing values need to copy it.

This crate is `#[no_std]` compatible.

```rust
use accessor::mapper::Mapper;

struct M;
impl Mapper for M {
    unsafe fn map(&mut self, phys_start: usize, bytes: usize) -> usize {
        unimplemented!()
    }

    fn unmap(&mut self, phys_start: usize, bytes: usize) {
        unimplemented!()
    }
}

// Create an accessor to an i32 value at the physical address 0x1000.
let mut a = unsafe {
    accessor::Single::<i32, M>::new(0x1000, M).expect("Failed to create an accessor.")
};

// Read a value.
a.read();

// Write a value.
a.write(3);

// Create an accessor to an array at the physical address 0x2000 of the type i32 that has 5 elements.
let mut arr = unsafe {
    accessor::Array::<i32, M>::new(0x2000, 5, M).expect("Failed to create an accessor.")
};

// Read the 2nd element.
arr.read_at(2);

// Write 42 as the 0th element.
arr.write_at(0, 42);
```

License: MIT OR Apache-2.0
