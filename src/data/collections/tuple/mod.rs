// devela::data::collections::tuple
//
//!
//

use crate::{self, Debug, Display, FmtResult, Formatter};

#[cfg(test)]
mod tests;

#[doc = crate::doc_private!()]
/// Marker trait to prevent downstream implementations of the [`Tuple`] trait.
trait Sealed {}

// NOTE: we import the generated code for the Tuple trait,
// and their implementations for tuples of many arities,
// As well as the enums TupleEnumRef and TupleEnumMut.
include!(concat!(env!("OUT_DIR"), "/build/tuple.rs"));

/// A formatting wrapper for [`Tuple`]s, implementing `Display` and `Debug`.
#[repr(transparent)]
pub struct TupleFmt<'a, T: Tuple>(&'a T);

#[doc = crate::doc_private!()]
/// Private trait for [`Tuple`]s with elements that implement `Debug`.
trait TupleDebug: Tuple {
    fn fmt_debug(&self, f: &mut Formatter) -> FmtResult<()>;
}
impl<T: TupleDebug> Debug for TupleFmt<'_, T> {
    fn fmt(&self, f: &mut Formatter) -> FmtResult<()> {
        self.0.fmt_debug(f)
    }
}

#[doc = crate::doc_private!()]
/// Private trait for [`Tuple`]s with elements that implement `Display`.
trait TupleDisplay: Tuple {
    fn fmt_display(&self, f: &mut Formatter) -> FmtResult<()>;
}
impl<T: TupleDisplay> Display for TupleFmt<'_, T> {
    fn fmt(&self, f: &mut Formatter) -> FmtResult<()> {
        self.0.fmt_display(f)
    }
}
