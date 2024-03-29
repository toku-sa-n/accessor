# accessor

[![GitHub Actions](https://github.com/toku-sa-n/accessor/workflows/Rust/badge.svg)](https://github.com/toku-sa-n/accessor/actions)
[![Crates.io](https://img.shields.io/crates/v/accessor)](https://crates.io/crates/accessor)
![Crates.io](https://img.shields.io/crates/l/accessor)
[![docs.rs](https://docs.rs/accessor/badge.svg)](https://docs.rs/accessor)

Accessors to access physical memory.

This crate provides accessors to values at a specific memory address. When an accessor is
created, physical memory is mapped to virtual memory. The methods of the accessors can access
a value at the specified physical address.  Once an accessor is dropped, the mapped memory is unmapped.

This crate is intended to access memory-mapped I/O. Reading and writing are done volatilely.

The accessed type must implement [`Copy`] because reading and writing values need to copy it.

This crate is `#[no_std]` compatible.

```rust,no_run
use accessor::array;
use accessor::mapper::Mapper;
use accessor::single;
use core::num::NonZeroUsize;

struct M;
impl Mapper for M {
    unsafe fn map(&mut self, phys_start: usize, bytes: usize) -> NonZeroUsize {
        todo!()
    }

    fn unmap(&mut self, phys_start: usize, bytes: usize) {
        todo!()
    }
}

// Create an accessor to an i32 value at the physical address 0x1000.
let mut a = unsafe { single::ReadWrite::<i32, M>::new(0x1000, M) };

// Read a value.
a.read_volatile();

// Write a value.
a.write_volatile(3);

// Create an accessor to an array at the physical address 0x2000 of the type i32 that has 5 elements.
let mut arr = unsafe { array::ReadWrite::<i32, M>::new(0x2000, 5, M) };

// Read the 2nd element.
arr.read_volatile_at(2);

// Write 42 as the 0th element.
arr.write_volatile_at(0, 42);
```

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

