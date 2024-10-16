// devela::data::collections::tuple
//
//!
//

use core::fmt::{self, Debug, Display};

#[cfg(test)]
mod tests;

// Marker trait to prevent downstream implementations of the `Tuple` trait.
trait Sealed {}

// NOTE: we import the generated code for the Tuple trait,
// and their implementations for tuples of many arities,
// As well as the enums TupleEnumRef and TupleEnumMut.
include!(concat!(env!("OUT_DIR"), "/build/tuple.rs"));

/// A formatting wrapper for [`Tuple`]s, implementing `Display` and `Debug`.
#[repr(transparent)]
pub struct TupleFmt<'a, T: Tuple>(&'a T);

// Private traits for tuples with elements that implement Debug or Display.
trait TupleDebug: Tuple {
    fn fmt_debug(&self, f: &mut fmt::Formatter) -> fmt::Result;
}
trait TupleDisplay: Tuple {
    fn fmt_display(&self, f: &mut fmt::Formatter) -> fmt::Result;
}

impl<T: TupleDebug> fmt::Debug for TupleFmt<'_, T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt_debug(f)
    }
}
impl<T: TupleDisplay> fmt::Display for TupleFmt<'_, T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt_display(f)
    }
}
