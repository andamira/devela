// devela::string::error
//
//!
//

use core::fmt;

/// The error type for strings backed by an array.
#[non_exhaustive]
#[derive(Debug)]
pub enum ArrayStringError {
    /// Not enough capacity for the attempted operation.
    ///
    /// Returns the needed capacity.
    NotEnoughCapacity(usize),

    /// Not enough elements for the attempted operation.
    ///
    /// Returns the needed number of elements.
    NotEnoughElements(usize),
}

impl fmt::Display for ArrayStringError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use ArrayStringError as SE;
        match self {
            SE::NotEnoughCapacity(c) => write!(f, "Not enough capacity. Needed: {c}"),
            SE::NotEnoughElements(e) => write!(f, "Not enough elements. Needed: {e}"),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for ArrayStringError {}

// Private result type for this module.
pub(super) type Result<T> = core::result::Result<T, ArrayStringError>;
