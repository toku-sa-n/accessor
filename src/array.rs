//! Accessor for an array

use crate::{error::Error, mapper::Mapper};
use core::{fmt, hash::Hash, marker::PhantomData, mem, ptr};

/// An accessor to read, modify, and write an array of some type on memory.
///
/// When accessing to an element of the array, the index starts from 0.
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
/// // Create an accessor to the array at the physical address 0x1000 that has 10 elements
/// // of i32 type.
/// let mut a = unsafe { accessor::Array::<u32, M>::new(0x1000, 10, mapper) };
///
/// // Read the 3rd element of the array.
/// a.read_volatile_at(3);
///
/// // Write 42 as the 5th element of the array.
/// a.write_volatile_at(5, 42);
///
/// // Update the 0th element.
/// a.update_volatile_at(0, |v| {
///     *v *= 2;
/// });
/// ```
pub struct Array<T, M>
where
    T: Copy,
    M: Mapper,
{
    virt: usize,
    len: usize,
    _marker: PhantomData<T>,
    mapper: M,
}
#[allow(clippy::len_without_is_empty)] // Array is never empty.
impl<T, M> Array<T, M>
where
    T: Copy,
    M: Mapper,
{
    /// Creates an accessor to `[T; len]` at the physical address `phys_base`.
    ///
    /// # Safety
    ///
    /// The caller must ensure the following conditions:
    /// - The array at the physical address `phys_base` is valid.
    /// - Any other accessors except the one returned by this method must not access the array
    /// while the returned one lives.
    ///
    /// # Panics
    ///
    /// This method panics if
    /// - `phys_base` is not aligned as the type `T` requires.
    /// - `len == 0`.
    pub unsafe fn new(phys_base: usize, len: usize, mut mapper: M) -> Self {
        assert!(super::is_aligned::<T>(phys_base));
        assert_ne!(len, 0);

        let bytes = mem::size_of::<T>() * len;
        let virt = mapper.map(phys_base, bytes).get();

        Self {
            virt,
            len,
            _marker: PhantomData,
            mapper,
        }
    }

    /// Creates an accessor to `[T; len]` at the physical address `phys_base`.
    ///
    /// # Safety
    ///
    /// The caller must ensure the following conditions:
    /// - The array at the physical address `phys_base` is valid.
    /// - Any other accessors except the one returned by this method must not access the array
    /// while the returned one lives.
    ///
    /// # Errors
    ///
    /// This method may return an error.
    /// - [`Error::NotAligned`] - `phys_base` is not aligned as the type `T` requires.
    /// - [`Error::EmptyArray`] - `len == 0`
    pub unsafe fn try_new(phys_base: usize, len: usize, mapper: M) -> Result<Self, Error> {
        if len == 0 {
            Err(Error::EmptyArray)
        } else if super::is_aligned::<T>(phys_base) {
            Ok(Self::new(phys_base, len, mapper))
        } else {
            Err(Error::NotAligned {
                alignment: mem::align_of::<T>(),
                address: phys_base,
            })
        }
    }

    /// Reads the `i`th element from the address that the accessor points.
    ///
    /// # Panics
    ///
    /// This method will panic if `i >= self.len()`
    pub fn read_volatile_at(&self, i: usize) -> T {
        assert!(i < self.len());

        // SAFETY: `Accessor::new_array` ensures that `self.addr(i)` is aligned properly.
        unsafe { ptr::read_volatile(self.addr(i) as *const _) }
    }

    /// Alias of [`Array::read_volatile_at`].
    #[deprecated(since = "0.3.1", note = "use `read_volatile_at`")]
    pub fn read_at(&self, i: usize) -> T {
        self.read_volatile_at(i)
    }

    /// Writes `v` as the `i`th element to the address that the accessor points to.
    ///
    /// # Panics
    ///
    /// This method will panic if `i >= self.len()`
    pub fn write_volatile_at(&mut self, i: usize, v: T) {
        assert!(i < self.len());

        // SAFETY: `Accessor::new_array` ensures that `self.addr(i)` is aligned properly.
        unsafe {
            ptr::write_volatile(self.addr(i) as *mut _, v);
        }
    }

    /// Alias of [`Array::write_volatile_at`].
    #[deprecated(since = "0.3.1", note = "use `write_volatile_at`")]
    pub fn write_at(&mut self, i: usize, v: T) {
        self.write_volatile_at(i, v);
    }

    /// Updates the `i`th element that the accessor points by reading it, modifying it, and writing it.
    pub fn update_volatile_at<U>(&mut self, i: usize, f: U)
    where
        U: FnOnce(&mut T),
    {
        let mut v = self.read_volatile_at(i);
        f(&mut v);
        self.write_volatile_at(i, v);
    }

    /// Alias of [`Array::update_volatile_at`].
    #[deprecated(since = "0.3.1", note = "use `update_volatile_at`")]
    pub fn update_at<U>(&mut self, i: usize, f: U)
    where
        U: FnOnce(&mut T),
    {
        self.update_volatile_at(i, f);
    }

    /// Returns the length of the array.
    pub fn len(&self) -> usize {
        self.len
    }

    fn addr(&self, i: usize) -> usize {
        self.virt + mem::size_of::<T>() * i
    }

    fn bytes(&self) -> usize {
        mem::size_of::<T>() * self.len
    }
}
impl<T, M> fmt::Debug for Array<T, M>
where
    T: Copy + fmt::Debug,
    M: Mapper,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self).finish()
    }
}
impl<T, M> PartialEq for Array<T, M>
where
    T: Copy + PartialEq,
    M: Mapper,
{
    fn eq(&self, other: &Self) -> bool {
        self.into_iter()
            .zip(other.into_iter())
            .map(|(a, b)| a.eq(&b))
            .all(|x| x)
    }
}
impl<T, M> Eq for Array<T, M>
where
    T: Copy + Eq,
    M: Mapper,
{
}
impl<T, M> Hash for Array<T, M>
where
    T: Copy + Hash,
    M: Mapper,
{
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        for e in self {
            e.hash(state);
        }
    }
}
impl<'a, T, M> IntoIterator for &'a Array<T, M>
where
    T: Copy,
    M: Mapper,
{
    type Item = T;
    type IntoIter = Iter<'a, T, M>;

    fn into_iter(self) -> Self::IntoIter {
        Iter::new(self)
    }
}
impl<T, M> Drop for Array<T, M>
where
    T: Copy,
    M: Mapper,
{
    fn drop(&mut self) {
        self.mapper.unmap(self.virt, self.bytes());
    }
}

/// An iterator over a value of `T`.
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Iter<'a, T, M>
where
    T: Copy,
    M: Mapper,
{
    a: &'a Array<T, M>,
    i: usize,
}
impl<'a, T, M> Iter<'a, T, M>
where
    T: Copy,
    M: Mapper,
{
    fn new(a: &'a Array<T, M>) -> Self {
        Self { a, i: 0 }
    }
}
impl<'a, T, M> Iterator for Iter<'a, T, M>
where
    T: Copy,
    M: Mapper,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.i < self.a.len() {
            let t = self.a.read_volatile_at(self.i);
            self.i += 1;
            Some(t)
        } else {
            None
        }
    }
}
