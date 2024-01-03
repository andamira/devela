// devela::num::int::trait
//
//!
//

#![allow(unused_imports)] // TEMP

use crate::num::{Num, NumErrors as Error, NumRef, NumResult as Result};
use core::ops::{Deref, DerefMut};

// mod impls; // TEMP

mod auto_impls {
    use super::{NumInt, NumRefInt};

    #[rustfmt::skip]
    impl<'a, T: NumInt> NumRefInt<'a> for &T {}
    #[rustfmt::skip]
    impl<'a, T: NumInt> NumRefInt<'a> for &mut T {}
}

/// Common trait for integer types.
///
/// This trait doesn't depend on having any operations implemented, and it
/// offers a common numeric API that returns [`NotImplemented`][Error::NotImplemented]
/// by default unless the methods are overriden.
///
/// Any concrete implementation must manually implement the operations it wants.
///
/// You could also ask for additional bounds like e.g. [`Add`][core::ops::Add].
///
/// Binary operations offer two alternative methods, one for when you want to
/// transfer ownership of the second element, and another one for when you don't.
/// Transferring ownership is more efficient for `Copy` types, and using a
/// reference is more appropriate for non-copy types.
///
/// For the default implementations we try to always offer a meaningful result,
/// even if the concrete type doesn't support it directly, we do the operation
/// on the underlying primitive and try to construct the new type again.
///
/// The standard library offers different methods for signed and unsigned types,
/// (e.g. abs, neg), and some are lacking for non-zero types (div, sub).
/// This trait try to offer the same methods everywhere and give a
/// result when a result is possible.
///
/// See also [`NumRefInt`] that is intended to be implemented for `Int` references.
#[cfg_attr(feature = "nightly", doc(cfg(feature = "num")))]
#[rustfmt::skip]
#[allow(unused_variables)]
pub trait NumInt: Num {}

/// Common trait for referenced integer types.
///
/// It is automatically implemented for references of types implementing [`NumInt`].
/// Mutable operations are only available for exclusive (`&mut`) references.
#[cfg_attr(feature = "nightly", doc(cfg(feature = "num")))]
#[rustfmt::skip]
#[allow(unused_variables)]
pub trait NumRefInt<'a>: NumRef<'a> where Self: Deref<Target = <Self as NumRef<'a>>::Own> {}
