# accessor

[![GitHub Actions](https://github.com/toku-sa-n/accessor/workflows/Rust/badge.svg)](https://github.com/toku-sa-n/accessor/actions)
[![Crates.io](https://img.shields.io/crates/v/accessor)](https://crates.io/crates/accessor)
![Crates.io](https://img.shields.io/crates/l/accessor)
[![docs.rs](https://docs.rs/accessor/badge.svg)](https://docs.rs/accessor)

Accessors to read and write physical memory volatilely, such as performing memory-mapped I/O.

This crate provides accessors to the value at a specific physical memory address.
The accessed type doesn't have to implement [`Copy`], but be aware that reading and writing the value creates a bitwise copy it.

Accessors are similar to pointers with volatile read/writes
(or for those who are familiar with crate [volatile(~v0.3.0)](https://docs.rs/volatile/0.3.0/volatile/index.html), pointers of volatile wrappers)
but also designed to refer correct physical addresses even in virtual memory mode,
once an appropriate physical-to-virtual memory mapper is specified.

When an accessor is created, the physical memory is mapped into virtual memory, with the help of the
mapper implemented by the crate user. The methods of accessors allow access to the value at the
specified physical address. Once an accessor is dropped, the mapped memory is unmapped.

If one has full control of physical memory addresses(e.g. developing their own kernel),
a 'virtual' address is equal to the physical one, and the mapper should map any address into itself.
The built-in mapper `mapper::Identity` can be used for such cases.

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

