//! Memory mapper module.

use core::num::NonZeroUsize;

/// A mapper trait for accessing physical memory.
pub trait Mapper {
    /// Maps `bytes` bytes of physical memory region starting from `phys_start` and returns the
    /// first virtual address.
    ///
    /// # Safety
    ///
    /// The caller must ensure that
    /// - no [`&mut`] references are aliased.
    /// - no values have invalid or uninitialized values.
    ///
    /// The caller must be careful, especially if it tries to remap by calling [`Mapper::unmap`], then
    /// [`Mapper::map`] to the same memory region.
    ///
    /// # Panics
    ///
    /// Depending on implementation, this method may panic if `phys_start` has null or any other
    /// invalid physical addresses.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use accessor::mapper::Mapper;
    ///
    /// unsafe fn map_pages<M>(m: &mut M, phys_start: usize, bytes: usize)
    /// where
    ///     M: Mapper
    /// {
    ///     let virt_start = m.map(phys_start, bytes);
    ///     println!("Physical address 0x{:X} is mapped to the virtual address 0x{:X}.",
    ///     phys_start, virt_start);
    /// }
    unsafe fn map(&mut self, phys_start: usize, bytes: usize) -> NonZeroUsize;

    /// Unmaps `bytes` bytes of the virtual memory region starting from `virt_start`.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use accessor::mapper::Mapper;
    ///
    /// fn unmap_pages<M>(m: &mut M, virt_start: usize, bytes: usize)
    /// where
    ///     M: Mapper,
    /// {
    ///     m.unmap(virt_start, bytes);
    ///     println!(
    ///         "Virtual memory region 0x{:X}..0x{:X} is unmapped.",
    ///         virt_start,
    ///         virt_start + bytes
    ///     );
    /// }
    /// ```
    fn unmap(&mut self, virt_start: usize, bytes: usize);
}

/// The trivial mapper, which maps an address into itself.
///
/// This mapper serves two purposes:
/// - It maps a physical address into the virtual address of the same value.
///   This is especially useful when you have an access to the physical address space
///   itself and should reference it directly, as when working on an OS kernel.
/// - It maps an already-mapped virtual address into itself, preventing duplicate
///   mapping calls to other non-trivial mappers.
///
/// An accessor of some type using this mapper may be regarded as a pointer to that type.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct Identity;
impl Mapper for Identity {
    unsafe fn map(&mut self, phys_base: usize, _bytes: usize) -> NonZeroUsize {
        NonZeroUsize::new(phys_base).expect("`phys_base` should not be null.")
    }
    fn unmap(&mut self, _virt_start: usize, _bytes: usize) {}
}
