//! Marker traits and enums.

/// A marker trait representing that the type implementing this can be used to specify the type of
/// an accessor (whether it can read a value, write a value, or both).
pub trait AccessorTypeSpecifier {}

/// A marker trait representing that the accessor can read a value.
pub trait Readable: AccessorTypeSpecifier {}

/// A marker trait representing that the accessor can write a value.
pub trait Writable: AccessorTypeSpecifier {}

/// A marker enum representing that the accessor can only read a value.
pub enum ReadOnly {}
impl AccessorTypeSpecifier for ReadOnly {}
impl Readable for ReadOnly {}

/// A marker enum representing that the accessor can only write a value.
pub enum WriteOnly {}
impl AccessorTypeSpecifier for WriteOnly {}
impl Writable for WriteOnly {}

/// A marker enum representing that the accessor can both read and write a value.
pub enum ReadWrite {}
impl AccessorTypeSpecifier for ReadWrite {}
impl Readable for ReadWrite {}
impl Writable for ReadWrite {}
