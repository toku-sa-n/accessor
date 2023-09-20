
we want to implement `#[set_array_indexing]` macro such that struct definition

```rust
#[repr(C)]
#[set_array_indexing]
pub struct Foo
{
    field_a: TypeA
    field_b: TypeB
    field_c: TypeB
}
```
expands into

```rust
impl<M, A> setarray::Generic<Foo, M, A>
where
    M: Mapper,
    A: AccessorTypeSpecifier,
{
    pub fn field_a_at<'a>(&'a self, i: usize) -> LifetimedGeneric<'a, TypeA, M, A> {
        assert!(i < self.len);
        let addr = self.virt + mem::size_of::<Foo>() * i + field_a_offset;
        LifetimedGeneric {
            access: single::Generic::new(addr, self.mapper.clone()),
            _lifetime: PhantomData
        }
    }
    pub fn field_b_at<'a>(&'a self, i: usize) -> LifetimedGeneric<'a, TypeB, M, A> {
        assert!(i < self.len);
        let addr = self.virt + mem::size_of::<Foo>() * i + field_b_offset;
        LifetimedGeneric {
            access: single::Generic::new(addr, self.mapper.clone()),
            _lifetime: PhantomData
        }
    }

    pub fn field_c_at<'a>(&'a self, i: usize) -> LifetimedGeneric<'a, TypeC, M, A> {
        assert!(i < self.len);
        let addr = self.virt + mem::size_of::<Foo>() * i + field_c_offset;
        LifetimedGeneric {
            access: single::Generic::new(addr, self.mapper.clone()),
            _lifetime: PhantomData
        }
    }
}
```