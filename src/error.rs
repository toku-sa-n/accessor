//! Errors

use core::fmt;

/// An enum representing errors.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum Error {
    /// The address passed as an argument is not aligned correctly.
    ///
    /// # Examples
    ///
    /// An error representing that the address 0x1001 is not 4 byte aligned.
    /// ```
    /// use accessor::error::Error;
    ///
    /// Error::NotAligned {
    ///     address: 0x1001,
    ///     alignment: 4,
    /// };
    /// ```
    NotAligned {
        /// The address passed as an argument.
        address: usize,
        /// The address must be `alignment` byte aligned.
        alignment: usize,
    },
    /// Attempted to create an empty array accessor.
    EmptyArray,
}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::NotAligned { alignment, address } => {
                write!(f, "Address 0x{address:X} is not {alignment} byte aligned.",)
            }
            Error::EmptyArray => write!(f, "Attempted to create an empty array accessor."),
        }
    }
}
