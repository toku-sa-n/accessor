//! Memory mapper module.

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
    /// The caller must be careful, especially if it tries to remap by calling [`unmap_pages`], then
    /// [`map_pages`] to the same memory region.
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
    unsafe fn map(&mut self, phys_start: usize, bytes: usize) -> usize;

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
