//! Accessor for an array

use {
    crate::{
        error::Error,
        mapper::Mapper,
        marker::{self, AccessorTypeSpecifier, Readable, Writable},
        single,
    },
    core::{fmt, hash::Hash, marker::PhantomData, mem, ptr},
};

/// An alis of [`Array`]
#[deprecated(since = "0.3.2", note = "Use `ReadWrite`.")]
pub type Array<T, M> = ReadWrite<T, M>;

/// A readable and writable accessor.
pub type ReadWrite<T, M> = Generic<T, M, marker::ReadWrite>;

/// A read-only accessor.
pub type ReadOnly<T, M> = Generic<T, M, marker::ReadOnly>;

/// A write-only accessor.
pub type WriteOnly<T, M> = Generic<T, M, marker::WriteOnly>;

/// Lifetimed wrapper of a single-element accessor.
/// The lifetime is set to the lifetime of its array accessor.
pub struct LifetimedGeneric<'a, T, M, A>
where
    M: Mapper,
    A: AccessorTypeSpecifier
{
    pub access: single::Generic<T, M, A>,
    _lifetime: PhantomData<&'a Generic<T, M, A>>
}

/// An accessor to read, modify, and write an array of some type on memory.
///
/// When accessing to an element of the array, the index starts from 0.
///
/// `T` does not need to implement [`Copy`]. However, be careful that [`Generic::read_volatile_at`]
/// creates and [`Generic::write_volatile_at`] writes a bitwise copy of a value.
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
///         todo!()
///     }
///
///     fn unmap(&mut self, phys_start: usize, bytes: usize) {
///         todo!()
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
/// 
/// // Below are the equivalent examples using `a.at()` method.
/// 
/// // Read the 3rd element of the array.
/// a.at(3).read_volatile();
/// 
/// // Write 42 as the 5th element of the array.
/// a.at(5).write_volatile(42);
/// 
/// // Update the 0th element.
/// a.at(0).update_volatile(|v| {
///     *v *= 2;
/// })
/// 
/// ```
pub struct Generic<T, M, A>
where
    M: Mapper,
    A: AccessorTypeSpecifier,
{
    virt: usize,
    len: usize,
    _marker: PhantomData<T>,
    _read_write: PhantomData<A>,
    mapper: M,
}
#[allow(clippy::len_without_is_empty)] // Array is never empty.
impl<T, M, A> Generic<T, M, A>
where
    M: Mapper,
    A: AccessorTypeSpecifier,
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
            _read_write: PhantomData,
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

    /// Returns `i`th element as a lifetimed single element accessor.
    pub fn at<'a>(&'a self, i: usize) -> LifetimedGeneric<'a, T, M, A> {
        assert!(i < self.len);
        unsafe {
            LifetimedGeneric {
                access: single::Generic::new(self.addr(i), self.mapper.clone()),
                _lifetime: PhantomData
            }
        }
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
impl<T, M, A> Generic<T, M, A>
where
    M: Mapper,
    A: Readable,
{
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

    /// Alias of [`Generic::read_volatile_at`].
    #[deprecated(since = "0.3.1", note = "use `read_volatile_at`")]
    pub fn read_at(&self, i: usize) -> T {
        self.read_volatile_at(i)
    }
}
impl<T, M, A> Generic<T, M, A>
where
    M: Mapper,
    A: Writable,
{
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

    /// Alias of [`Generic::write_volatile_at`].
    #[deprecated(since = "0.3.1", note = "use `write_volatile_at`")]
    pub fn write_at(&mut self, i: usize, v: T) {
        self.write_volatile_at(i, v);
    }
}
impl<T, M, A> Generic<T, M, A>
where
    M: Mapper,
    A: Readable + Writable,
{
    /// Updates the `i`th element that the accessor points by reading it, modifying it, and writing it.
    pub fn update_volatile_at<U>(&mut self, i: usize, f: U)
    where
        U: FnOnce(&mut T),
    {
        let mut v = self.read_volatile_at(i);
        f(&mut v);
        self.write_volatile_at(i, v);
    }

    /// Alias of [`Generic::update_volatile_at`].
    #[deprecated(since = "0.3.1", note = "use `update_volatile_at`")]
    pub fn update_at<U>(&mut self, i: usize, f: U)
    where
        U: FnOnce(&mut T),
    {
        self.update_volatile_at(i, f);
    }
}
impl<T, M, A> fmt::Debug for Generic<T, M, A>
where
    T: fmt::Debug,
    M: Mapper,
    A: Readable,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self).finish()
    }
}
impl<T, M, A> PartialEq for Generic<T, M, A>
where
    T: PartialEq,
    M: Mapper,
    A: Readable,
{
    fn eq(&self, other: &Self) -> bool {
        self.into_iter()
            .zip(other.into_iter())
            .map(|(a, b)| a.eq(&b))
            .all(|x| x)
    }
}
impl<T, M, A> Eq for Generic<T, M, A>
where
    T: Eq,
    M: Mapper,
    A: Readable,
{
}
impl<T, M, A> Hash for Generic<T, M, A>
where
    T: Hash,
    M: Mapper,
    A: Readable,
{
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        for e in self {
            e.hash(state);
        }
    }
}
impl<'a, T, M, A> IntoIterator for &'a Generic<T, M, A>
where
    M: Mapper,
    A: Readable,
{
    type Item = T;
    type IntoIter = Iter<'a, T, M, A>;

    fn into_iter(self) -> Self::IntoIter {
        Iter::new(self)
    }
}
impl<T, M, A> Drop for Generic<T, M, A>
where
    M: Mapper,
    A: AccessorTypeSpecifier,
{
    fn drop(&mut self) {
        self.mapper.unmap(self.virt, self.bytes());
    }
}

/// An iterator over a value of `T`.
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Iter<'a, T, M, A>
where
    M: Mapper,
    A: Readable,
{
    a: &'a Generic<T, M, A>,
    i: usize,
}
impl<'a, T, M, A> Iter<'a, T, M, A>
where
    M: Mapper,
    A: Readable,
{
    fn new(a: &'a Generic<T, M, A>) -> Self {
        Self { a, i: 0 }
    }
}
impl<'a, T, M, A> Iterator for Iter<'a, T, M, A>
where
    M: Mapper,
    A: Readable,
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
