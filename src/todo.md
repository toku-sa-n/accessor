
# specifying (field, index) at the same time

we want to implement `#[set_array_indexing]` macro such that struct definition

```rust
#[repr(C)]
#[set_array_indexing]
pub struct Foo {
    pub field_a: TypeA
    pub field_b: TypeB
    pub field_c: TypeB
}
```

expands into
(note: using `memoffset::offset_of` macro hereafter)

```rust
impl<M, A> setarray::Generic<Foo, M, A>
where
    M: Mapper,
    A: AccessorTypeSpecifier,
{
    pub fn field_a_at<'a>(&'a self, i: usize) -> LifetimedGeneric<'a, TypeA, M, A> {
        assert!(i < self.len);
        let addr = self.virt + mem::size_of::<Foo>() * i + offset_of!(Foo, field_a);
        unsafe {
            LifetimedGeneric {
                access: single::Generic::new(addr, self.mapper.clone()),
                _lifetime: PhantomData
            }
        }
    }
    pub fn field_b_at<'a>(&'a self, i: usize) -> LifetimedGeneric<'a, TypeB, M, A> {
        assert!(i < self.len);
        let addr = self.virt + mem::size_of::<Foo>() * i + offset_of!(Foo, field_b);
        unsafe {
            LifetimedGeneric {
                access: single::Generic::new(addr, self.mapper.clone()),
                _lifetime: PhantomData
            }
        }
    }

    pub fn field_c_at<'a>(&'a self, i: usize) -> LifetimedGeneric<'a, TypeC, M, A> {
        assert!(i < self.len);
        let addr = self.virt + mem::size_of::<Foo>() * i + offset_of!(Foo, field_c);
        unsafe {
            LifetimedGeneric {
                access: single::Generic::new(addr, self.mapper.clone()),
                _lifetime: PhantomData
            }
        }
    }
}
```

# specifying index first, and then field

This type definition

```rust
#[repr(C)]
pub struct Foo {
    pub field_a: TypeA
    pub field_b: TypeB
    pub field_c: TypeC
}
```

with some appropriate macro, it derives

```rust
pub struct LifetimedSetGenericOfFoo<'a, M, A>
where
    M: Mapper,
    A: AccessorTypeSpecifier
{ // if it can't be generic, generate a name like `LifetimedSetGenericFoo` or something.
    pub field_a: single::Generic<TypeA, M, A>,
    pub field_b: single::Generic<TypeB, M, A>,
    pub field_c: single::Generic<TypeC, M, A>,
    _lifetime: PhantomData<&'a setarray::Generic<Foo, M, A>>
}

impl<M, A> setarray::Generic<Foo, M, A>
where
    M: Mapper,
    A: AccessorTypeSpecifier,
{
    pub fn at<'a>(&'a self, i: usize) -> LifetimedSetGenericOfFoo<'a, M, A> {
        assert!(i < self.len);
        let addr = self.virt + mem::size_of::<Foo>() * i;
        LifetimedSetGenericOfFoo {
            field_a: single::Generic::new(addr + offset_of!(Foo, field_a), self.mapper.clone()),
            field_b: single::Generic::new(addr + offset_of!(Foo, field_b), self.mapper.clone()),
            field_c: single::Generic::new(addr + offset_of!(Foo, field_c), self.mapper.clone()),
            _lifetime: PhantomData
        }
    }
}
```



```sh
RUSTFLAGS="-Z macro-backtrace" cargo check
```