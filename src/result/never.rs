// devela::result::never
//
//! `never` types.
//

use crate::code::reexport;
use core::fmt;

reexport! { rust: core::convert,
doc: "The type intended for a [`Result::Err`] variant that can never happen.",
@Infallible as NeverErr}

/// The type intended for a [`Result::Ok`] variant that can never happen.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum NeverOk {}

impl fmt::Display for NeverOk {
    fn fmt(&self, _: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {}
    }
}
