//! Accessor for an array

use {
    crate::{
        error::Error,
        mapper::{Identity, Mapper},
        marker::{self, AccessorTypeSpecifier, Readable, Writable},
        single,
    },
    core::{fmt, hash::Hash, marker::PhantomData, mem, ptr},
};

/// An alias of [`Array`]
#[deprecated(since = "0.3.2", note = "Use `ReadWrite`.")]
pub type Array<T, M> = ReadWrite<T, M>;

/// A readable and writable accessor.
pub type ReadWrite<T, M> = Generic<T, M, marker::ReadWrite>;

/// A read-only accessor.
pub type ReadOnly<T, M> = Generic<T, M, marker::ReadOnly>;

/// A write-only accessor.
pub type WriteOnly<T, M> = Generic<T, M, marker::WriteOnly>;

/// Bounded wrapper of a single-element accessor.
/// The lifetime is set to the lifetime of its array accessor.
pub struct Bounded<'a, T, M, A>
where
    M: Mapper,
    A: AccessorTypeSpecifier,
{
    a: single::Generic<T, Identity, A>,
    _lifetime: PhantomData<&'a Generic<T, M, A>>,
}

impl<'a, T, M, A> Bounded<'a, T, M, A>
where
    M: Mapper,
    A: Readable,
{
    /// Reads a value from the address that the accessor points to.
    ///
    /// # Panics
    ///
    /// This method will panic if `i >= self.len()`.
    #[must_use]
    pub fn read_volatile(&self) -> T {
        self.a.read_volatile()
    }

    /// Alias of [`Bounded::read_volatile`].
    #[deprecated(since = "0.3.1", note = "use `read_volatile`")]
    #[must_use]
    pub fn read(&self) -> T {
        self.read_volatile()
    }
}
impl<'a, T, M, A> Bounded<'a, T, M, A>
where
    M: Mapper,
    A: Writable,
{
    /// Writes a value to the address that the accessor points to.
    ///
    /// # Panics
    ///
    /// This method will panic if `i >= self.len()`.
    pub fn write_volatile(&mut self, v: T) {
        self.a.write_volatile(v);
    }

    /// Alias of [`Bounded::write_volatile`].
    #[deprecated(since = "0.3.1", note = "use `write_volatile`")]
    pub fn write(&mut self, v: T) {
        self.write_volatile(v);
    }
}
impl<'a, T, M, A> Bounded<'a, T, M, A>
where
    M: Mapper,
    A: Readable + Writable,
{
    /// Updates a value that the accessor points to by reading it, modifying it, and writing it.
    ///
    /// Note that some MMIO regions (e.g. the Command Ring Pointer field of the Command
    /// Ring Control Register of the xHCI) may return 0 regardless of the actual values of the
    /// fields. For these regions, this operation should be called only once.
    ///
    /// # Panics
    ///
    /// This method will panic if `i >= self.len()`.
    pub fn update_volatile<U>(&mut self, f: U)
    where
        U: FnOnce(&mut T),
    {
        self.a.update_volatile(f);
    }

    /// Alias of [`Bounded::update_volatile`].
    #[deprecated(since = "0.3.1", note = "use `update_volatile`")]
    pub fn update<U>(&mut self, f: U)
    where
        U: FnOnce(&mut T),
    {
        self.update_volatile(f);
    }
}

/// Combined with proc-macro [`BoundedStructuralOf`], this trait converts array accessors of field struct types into a struct of accessors with same field names.
///
/// This trait is intended to be implemented automatically by [`BoundedStructuralOf`] macro expansion. Users should not implement this manually.
///
/// # Examples
///
/// ```no_run
/// use accessor::mapper::Identity;
/// use accessor::BoundedStructuralOf;
/// use accessor::array::{BoundedStructural, BoundedStructuralMut};
///
/// #[repr(C)]
/// #[derive(Clone, Copy, BoundedStructuralOf)]
/// struct Foo {
///     x: u32,
///     y: u32,
/// }
///
/// // The above derivation creates a struct-of-accessor type called `BoundedStructuralOfFoo` which is roughly equivalent to:
/// // ```
/// // struct BoundedStructuralOfFoo {
/// //     x: accessor::single::ReadWrite::<u32, Identity>,
/// //     y: accessor::single::ReadWrite::<u32, Identity>,
/// // }
/// // ```
/// // The derivation also implements `BoundedStructural<Foo, M, A>` and `BoundedStructuralMut<Foo, M, A>` so that an `accessor::array::ReadWrite::<Foo, M>` instance
/// // can be indexed into a `BoundedStructuralOfFoo` item, which has a lifetime bound to the base array accessor.
///
/// let mut a = unsafe { accessor::array::ReadWrite::<Foo, M>::new(0x1000, 10, Identity) };
///
/// // read `x` field of 0th element of the array.
/// let x = a.structural_at(0).x.read_volatile();
///
/// // write 5 as the `y` field of 2nd element of the array.
/// a.structural_at_mut(2).y.write_volatile(5);
///
/// ```
///
pub trait BoundedStructural<T, M, A>
where
    M: Mapper,
    A: Readable,
{
    /// The concrete type of the struct of accessors which `.structural_at(i)` returns.
    type BoundedStructuralType<'a>
    where
        Self: 'a;

    /// Returns `i`th element as a bounded struct of read-only accessors.
    fn structural_at(&self, i: usize) -> Self::BoundedStructuralType<'_>;
}

/// The mutable counterpart for [`BoundedStructural`].
/// See [`BoundedStructural`] for details.
pub trait BoundedStructuralMut<T, M, A>
where
    M: Mapper,
    A: Writable,
{
    /// The concrete type of the struct of accessors which `.structural_at_mut(i)` returns.
    type BoundedStructuralType<'a>
    where
        Self: 'a;

    /// Returns `i`th element as a bounded struct of writable accessors.
    fn structural_at_mut(&mut self, i: usize) -> Self::BoundedStructuralType<'_>;
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
/// let mut a = unsafe { accessor::array::ReadWrite::<u32, M>::new(0x1000, 10, mapper) };
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
/// // Below are the equivalent examples using `.at()` and `.at_mut()` method.
///
/// // Read the 3rd element of the array.
/// a.at(3).read_volatile();
///
/// // Write 42 as the 5th element of the array.
/// a.at_mut(5).write_volatile(42);
///
/// // Update the 0th element.
/// a.at_mut(0).update_volatile(|v| {
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

    /// Create an element accessor for specific index of this array.
    ///
    /// Use this method if you need the ownership of the indexed accessor,
    /// and are sure that you will not use the original array accessor again.
    /// Otherwise, consider `.at(i)`, `.structural_at(i)` or their mutable counterparts.
    ///
    /// # Safety
    ///
    /// The caller must ensure the following conditions:
    /// - The array accessor should live longer than the element accessor.
    /// - After an element accessor has been created, the array accessor should not access into index `i`
    ///   including creating a new accessor for the same index `i`.
    ///
    /// # Panics
    ///
    /// This method will panic if `i >= self.len()`.
    pub unsafe fn unbounded_at(&self, i: usize) -> single::Generic<T, Identity, A> {
        assert!(i < self.len);
        single::Generic::new(self.addr(i), Identity)
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

    /// Returns the length of the array.
    pub fn len(&self) -> usize {
        self.len
    }

    /// Returns the virtual address of the item of index `i`.
    ///
    /// This is public but hidden, since this method should be called in `accessor_macros::BoundedStructuralOf` proc-macro expansion.
    /// Users of this crate are not intended to call this directly.
    #[doc(hidden)]
    pub unsafe fn addr(&self, i: usize) -> usize {
        self.virt + mem::size_of::<T>() * i
    }
}

impl<T, M, A> Generic<T, M, A>
where
    M: Mapper,
    A: Readable,
{
    /// Returns `i`th element as a read-only bound single element accessor.
    ///
    /// # Panics
    ///
    /// This method will panic if `i >= self.len()`.
    pub fn at(&self, i: usize) -> Bounded<'_, T, M, marker::ReadOnly> {
        assert!(i < self.len);
        unsafe {
            Bounded {
                a: single::Generic::new(self.addr(i), Identity),
                _lifetime: PhantomData,
            }
        }
    }

    /// Reads the `i`th element from the address that the accessor points to.
    ///
    /// `accessor.read_volatile_at(i)` is equivalent to `accessor.at(i).read_volatile()`.
    ///
    /// # Panics
    ///
    /// This method will panic if `i >= self.len()`.
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
    /// Returns `i`th element as a writable bound single element accessor.
    ///
    /// # Panics
    ///
    /// This method will panic if `i >= self.len()`.
    pub fn at_mut(&mut self, i: usize) -> Bounded<'_, T, M, A> {
        assert!(i < self.len);
        unsafe {
            Bounded {
                a: single::Generic::new(self.addr(i), Identity),
                _lifetime: PhantomData,
            }
        }
    }

    /// Writes `v` as the `i`th element to the address that the accessor points to.
    ///
    /// `accessor.write_volatile_at(i, v)` is equivalent to `accessor.at_mut(i).write_volatile(v)`.
    ///
    /// # Panics
    ///
    /// This method will panic if `i >= self.len()`.
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
    ///
    /// `accessor.update_volatile_at(i, f)` is equivalent to `accessor.at_mut(i).update_volatile(f)`.
    ///
    /// # Panics
    ///
    /// This method will panic if `i >= self.len()`.
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
            .zip(other)
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
        let bytes = mem::size_of::<T>() * self.len;
        self.mapper.unmap(self.virt, bytes);
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

// TODO: Rewrite the following tests as doc tests once
// https://github.com/rust-lang/rust/issues/78695 is fixed.
#[cfg(test)]
mod tests {
    use super::*;
    use core::num::NonZeroUsize;

    struct M;
    impl Mapper for M {
        unsafe fn map(&mut self, phys_start: usize, _: usize) -> NonZeroUsize {
            NonZeroUsize::new(phys_start).unwrap()
        }

        fn unmap(&mut self, _: usize, _: usize) {}
    }

    #[test]
    fn test_read_volatile_at() {
        let arr = [1, 2, 3, 4, 5];
        let a = unsafe { ReadOnly::<u32, _>::new(base_addr(&arr), arr.len(), M) };

        assert_eq!(a.read_volatile_at(0), 1);
    }

    #[test]
    fn test_write_volatile_at() {
        let mut arr = [1, 2, 3, 4, 5];
        let mut a = unsafe { WriteOnly::<u32, _>::new(base_addr(&mut arr), arr.len(), M) };

        a.write_volatile_at(0, 42);
        assert_eq!(arr[0], 42);
    }

    #[test]
    fn test_update_volatile_at() {
        let mut arr = [1, 2, 3, 4, 5];
        let mut a = unsafe { ReadWrite::<u32, _>::new(base_addr(&mut arr), arr.len(), M) };

        a.update_volatile_at(0, |v| {
            *v *= 2;
        });
        assert_eq!(arr[0], 2);
    }

    #[test]
    fn test_into_iter() {
        let arr = [1, 2, 3, 4, 5];
        let a = unsafe { ReadOnly::<u32, _>::new(base_addr(&arr), arr.len(), M) };

        let mut iter = a.into_iter();
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(4));
        assert_eq!(iter.next(), Some(5));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_eq() {
        let arr = [1, 2, 3, 4, 5];
        let a = unsafe { ReadOnly::<u32, _>::new(base_addr(&arr), arr.len(), M) };
        let b = unsafe { ReadOnly::<u32, _>::new(base_addr(&arr), arr.len(), M) };

        assert_eq!(a, b);
    }

    #[test]
    #[should_panic]
    fn test_out_of_bounds() {
        let arr = [1, 2, 3, 4, 5];
        let a = unsafe { ReadOnly::<u32, _>::new(base_addr(&arr), arr.len(), M) };

        let _ = a.read_volatile_at(5);
    }

    #[test]
    #[should_panic]
    fn test_empty_array() {
        let arr = [1, 2, 3, 4, 5];
        let _ = unsafe { ReadOnly::<u32, _>::new(base_addr(&arr), 0, M) };
    }

    #[test]
    #[should_panic]
    fn test_not_aligned() {
        let arr = [1, 2, 3, 4, 5];
        let _ = unsafe { ReadOnly::<u32, _>::new(base_addr(&arr) + 1, arr.len(), M) };
    }

    #[test]
    fn test_err_empty_array() {
        let arr = [1, 2, 3, 4, 5];
        let r = unsafe { ReadOnly::<u32, _>::try_new(base_addr(&arr), 0, M) };

        assert_eq!(r, Err(Error::EmptyArray));
    }

    #[test]
    fn test_err_not_aligned() {
        let arr = [1, 2, 3, 4, 5];
        let r = unsafe { ReadOnly::<u32, _>::try_new(base_addr(&arr) + 1, arr.len(), M) };

        assert_eq!(
            r,
            Err(Error::NotAligned {
                alignment: mem::align_of::<u32>(),
                address: base_addr(&arr) + 1,
            })
        );
    }

    fn base_addr<T>(a: &[T]) -> usize {
        return a.as_ptr() as usize;
    }
}
