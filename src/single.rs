//! An accessor to a single element

use crate::{error::Error, mapper::Mapper};
use core::{convert::TryInto, fmt, marker::PhantomData, mem, ptr};

/// An accessor to read, modify, and write a single value of memory.
///
/// # Examples
///
/// ```no_run
/// use accessor::mapper::Mapper;
///
/// struct M;
/// impl Mapper for M {
///     unsafe fn map(&mut self, phys_start: usize, bytes: usize) -> usize {
///         unimplemented!()
///     }
///
///     fn unmap(&mut self, phys_start: usize, bytes: usize) {
///         unimplemented!()
///     }
/// }
///
/// let mapper = M;
///
/// // Creates an accessor to the i32 value at the physical address 0x1000.
/// let mut a = unsafe {
///     accessor::Single::<i32, M>::new(0x1000, mapper).expect("Failed to create an accessor.")
/// };
///
/// // Read a value.
/// a.read();
///
/// // Write 42.
/// a.write(42);
///
/// // Update the value.
/// a.update(|v| {
///     *v *= 2;
/// });
/// ```
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Single<T, M>
where
    T: Copy,
    M: Mapper,
{
    virt: usize,
    bytes: usize,
    _marker: PhantomData<T>,
    mapper: M,
}
impl<T, M> Single<T, M>
where
    T: Copy,
    M: Mapper,
{
    /// Creates a new accessor. The element is at the physical address `phys_base`.
    ///
    /// # Safety
    ///
    /// Caller must ensure that only one accessor to the same region is created, otherwise
    /// it may cause undefined behaviors such as data race.
    pub unsafe fn new(phys_base: usize, mapper: M) -> Result<Self, Error> {
        if super::is_aligned::<T>(phys_base) {
            Ok(Self::new_aligned(phys_base, mapper))
        } else {
            Err(Error::NotAligned {
                alignment: mem::align_of::<T>().try_into().unwrap(),
                address: (phys_base).try_into().unwrap(),
            })
        }
    }

    /// Reads a value from where the accessor points.
    pub fn read(&self) -> T {
        // SAFETY: `Accessor::new` ensures that `self.virt` is aligned properly.
        unsafe { ptr::read_volatile(self.virt as *const _) }
    }

    /// Writes a value to where the accessor points.
    pub fn write(&mut self, v: T) {
        // SAFETY: `Accessor::new` ensures that `self.virt` is aligned properly.
        unsafe { ptr::write_volatile(self.virt as *mut _, v) }
    }

    /// Updates a value which the accessor points by reading it, modifying it, and writing it.
    ///
    /// Note that some MMIO region (e.g. the Command Ring Pointer field of the Command
    /// Ring Control Register of the xHCI) may return 0 regardless of the actual values of the
    /// fields. For these regions, this operation should be called only once.
    pub fn update<U>(&mut self, f: U)
    where
        U: FnOnce(&mut T),
    {
        let mut v = self.read();
        f(&mut v);
        self.write(v);
    }

    /// # Safety
    ///
    /// Caller must ensure that only one accessor to the same region is created, otherwise
    /// it may cause undefined behaviors such as data race.
    unsafe fn new_aligned(phys_base: usize, mut mapper: M) -> Self {
        assert!(super::is_aligned::<T>(phys_base));

        let bytes = mem::size_of::<T>();
        let virt = mapper.map(phys_base, bytes);

        Self {
            virt,
            bytes,
            _marker: PhantomData,
            mapper,
        }
    }
}
impl<T, M> fmt::Debug for Single<T, M>
where
    T: Copy + fmt::Debug,
    M: Mapper,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.read())
    }
}
impl<T, M> Drop for Single<T, M>
where
    T: Copy,
    M: Mapper,
{
    fn drop(&mut self) {
        self.mapper.unmap(self.virt, self.bytes);
    }
}
