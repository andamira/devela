// devela::result::never
//
//! `never` types.
//

use core::fmt;

/// The type intended for a [`Result::Err`] variant that can never happen.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum NeverErr {}

/// The type intended for a [`Result::Ok`] variant that can never happen.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum NeverOk {}

#[cfg(feature = "std")]
impl std::error::Error for NeverErr {}

impl fmt::Display for NeverErr {
    fn fmt(&self, _: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {}
    }
}

impl fmt::Display for NeverOk {
    fn fmt(&self, _: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {}
    }
}
