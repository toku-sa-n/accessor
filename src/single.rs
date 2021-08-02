//! An accessor to a single element

use crate::{error::Error, mapper::Mapper};
use core::{fmt, hash::Hash, marker::PhantomData, mem, ptr};

/// An accessor to read, modify, and write a single value of memory.
///
/// # Examples
///
/// ```no_run
/// use accessor::mapper::Mapper;
/// use core::num::NonZeroUsize;
///
/// struct M;
/// impl Mapper for M {
///     unsafe fn map(&mut self, phys_start: usize, bytes: usize) -> NonZeroUsize {
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
/// // Create an accessor to the i32 value at the physical address 0x1000.
/// let mut a = unsafe { accessor::Single::<i32, M>::new(0x1000, mapper) };
///
/// // Read a value.
/// a.read_volatile();
///
/// // Write 42.
/// a.write_volatile(42);
///
/// // Update the value.
/// a.update_volatile(|v| {
///     *v *= 2;
/// });
/// ```
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
    /// Creates a new accessor to an element of type `T` at the physical address `phys_base`.
    ///
    /// # Safety
    ///
    /// The caller must ensure the following conditions:
    /// - The value at the physical address `phys_base` is valid.
    /// - Any other accessors except the one returned by this method must not access the value
    /// while the returned one lives.
    ///
    /// # Panics
    ///
    /// This method panics if `phys_base` is not aligned as the type `T` requires.
    pub unsafe fn new(phys_base: usize, mut mapper: M) -> Self {
        assert!(super::is_aligned::<T>(phys_base));

        let bytes = mem::size_of::<T>();
        let virt = mapper.map(phys_base, bytes).get();

        Self {
            virt,
            bytes,
            _marker: PhantomData,
            mapper,
        }
    }

    /// Creates a new accessor to an element of type `T` at the physical address `phys_base`.
    ///
    /// # Safety
    ///
    /// The caller must ensure the following conditions:
    /// - The value at the physical address `phys_base` is valid.
    /// - Any other accessors except the one returned by this method must not access the value
    /// while the returned one lives.
    ///
    /// # Errors
    ///
    /// This method may return a [`Error::NotAligned`] error if `phys_base` is not aligned as the
    /// type `T` requires.
    pub unsafe fn try_new(phys_base: usize, mapper: M) -> Result<Self, Error> {
        if super::is_aligned::<T>(phys_base) {
            Ok(Self::new(phys_base, mapper))
        } else {
            Err(Error::NotAligned {
                alignment: mem::align_of::<T>(),
                address: phys_base,
            })
        }
    }

    /// Reads a value from the address that the accessor points to.
    pub fn read_volatile(&self) -> T {
        // SAFETY: `Accessor::new` ensures that `self.virt` is aligned properly.
        unsafe { ptr::read_volatile(self.virt as *const _) }
    }

    /// Alias of [`Single::read_volatile`].
    #[deprecated(note = "use `read_volatile`")]
    pub fn read(&self) -> T {
        self.read_volatile()
    }

    /// Writes a value to the address that the accessor points to.
    pub fn write_volatile(&mut self, v: T) {
        // SAFETY: `Accessor::new` ensures that `self.virt` is aligned properly.
        unsafe {
            ptr::write_volatile(self.virt as *mut _, v);
        }
    }

    /// Alias of [`Single::write_volatile`].
    #[deprecated(note = "use `write_volatile`")]
    pub fn write(&mut self, v: T) {
        self.write_volatile(v);
    }

    /// Updates a value that the accessor points by reading it, modifying it, and writing it.
    ///
    /// Note that some MMIO regions (e.g. the Command Ring Pointer field of the Command
    /// Ring Control Register of the xHCI) may return 0 regardless of the actual values of the
    /// fields. For these regions, this operation should be called only once.
    pub fn update_volatile<U>(&mut self, f: U)
    where
        U: FnOnce(&mut T),
    {
        let mut v = self.read_volatile();
        f(&mut v);
        self.write_volatile(v);
    }

    /// Alias of [`Single::update_volatile`].
    #[deprecated(note = "use `update_volatile`")]
    pub fn update<U>(&mut self, f: U)
    where
        U: FnOnce(&mut T),
    {
        self.update_volatile(f);
    }
}
impl<T, M> fmt::Debug for Single<T, M>
where
    T: Copy + fmt::Debug,
    M: Mapper,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.read_volatile())
    }
}
impl<T, M> PartialEq for Single<T, M>
where
    T: Copy + PartialEq,
    M: Mapper,
{
    fn eq(&self, other: &Self) -> bool {
        self.read_volatile().eq(&other.read_volatile())
    }
}
impl<T, M> Eq for Single<T, M>
where
    T: Copy + Eq,
    M: Mapper,
{
}
impl<T, M> PartialOrd for Single<T, M>
where
    T: Copy + PartialOrd,
    M: Mapper,
{
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        self.read_volatile().partial_cmp(&other.read_volatile())
    }
}
impl<T, M> Ord for Single<T, M>
where
    T: Copy + Ord,
    M: Mapper,
{
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.read_volatile().cmp(&other.read_volatile())
    }
}
impl<T, M> Hash for Single<T, M>
where
    T: Copy + Hash,
    M: Mapper,
{
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.read_volatile().hash(state);
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
