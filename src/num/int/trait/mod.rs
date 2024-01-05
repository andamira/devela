// devela::num::int::trait
//
//!
//

use crate::num::{Num, NumErrors as E, NumRef, NumResult as Result};
use core::ops::{Deref, DerefMut};

mod impls;

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
/// offers a common numeric API that returns [`NotImplemented`][E::NotImplemented]
/// by default unless the methods are overriden.
///
/// Any concrete implementation must manually implement the operations it wants.
///
/// Most methods come in two variants, starting with different prefixes:
/// - `int_*`: takes the arguments by value.
/// - `int_ref_*`: takes the arguments by reference.
///
/// For all default implementations we try to always offer a meaningful result,
/// even if the concrete type doesn't support it directly, we do the operation
/// on the underlying primitive and try to construct the new type again.
///
/// The standard library offers different methods for signed and unsigned types,
/// (e.g. abs, neg), and some are lacking for non-zero types (div, sub).
/// This trait try to offer the same methods everywhere and give a
/// result when a result is possible.
///
/// See also [`NumRefInt`] that is automatically implemented for `NumInt` references.
#[cfg_attr(feature = "nightly", doc(cfg(feature = "num")))]
#[rustfmt::skip] #[allow(unused_variables)]
pub trait NumInt: Num {
    /* digital root */

    /// Returns the digital root in base 10.
    /// - Takes arguments by value.
    fn int_digital_root(self) -> Result<Self::Out> where Self: Sized{ E::ni() }
    /// Returns the digital root in base 10.
    /// - Takes arguments by reference.
    fn int_ref_digital_root(&self) -> Result<Self::Out> where Self: Sized{ E::ni() }

    /// Returns the digital root in the given `base`.
    /// - Takes arguments by value.
    fn int_digital_root_base(self, base: Self::Rhs) -> Result<Self::Out>
        where Self: Sized{ E::ni() }
    /// Returns the digital root in the given `base`.
    /// - Takes arguments by reference.
    fn int_ref_digital_root_base(&self, base: &Self::Rhs) -> Result<Self::Out>
        where Self: Sized{ E::ni() }

    /* digits */

    /// Returns the number of digits in base 10.
    /// - Takes arguments by value.
    fn int_digits(self) -> Result<Self::Out> where Self: Sized{ E::ni() }
    /// Returns the number of digits in base 10.
    /// - Takes arguments by reference.
    fn int_ref_digits(&self) -> Result<Self::Out> where Self: Sized{ E::ni() }

    /// Returns the number of digits in base 10 including the negative sign.
    /// - Takes arguments by value.
    fn int_digits_sign(self) -> Result<Self::Out> where Self: Sized{ E::ni() }
    /// Returns the number of digits in base 10 including the negative sign.
    /// - Takes arguments by reference.
    fn int_ref_digits_sign(&self) -> Result<Self::Out> where Self: Sized{ E::ni() }

    /// Returns the number of digits in the given `base`.
    /// - Takes arguments by value.
    fn int_digits_base(self, base: Self::Rhs) -> Result<Self::Out> where Self: Sized{ E::ni() }
    /// Returns the number of digits in the given `base`.
    /// - Takes arguments by reference.
    fn int_ref_digits_base(&self, base: &Self::Rhs) -> Result<Self::Out>
        where Self: Sized{ E::ni() }

    /// Returns the number of digits in the given `base`.
    /// - Takes arguments by value.
    fn int_digits_base_sign(self, base: Self::Rhs) -> Result<Self::Out> where Self: Sized{ E::ni() }
    /// Returns the number of digits in the given `base`.
    /// - Takes arguments by reference.
    fn int_ref_digits_base_sign(&self, base: &Self::Rhs) -> Result<Self::Out>
        where Self: Sized{ E::ni() }

    /* gcd & lcm */

    /// Returns the <abbr title="Greatest Common Divisor">GCD</abbr>.
    /// - Takes arguments by value.
    fn int_gcd(self, other: Self::Rhs) -> Result<Self::Out> where Self: Sized{ E::ni() }
    /// Returns the <abbr title="Greatest Common Divisor">GCD</abbr>.
    /// - Takes arguments by reference.
    fn int_ref_gcd(&self, other: &Self::Rhs) -> Result<Self::Out> where Self: Sized{ E::ni() }

    /// Returns the <abbr title="Greatest Common Divisor">GCD</abbr> and the Bézout coeficients.
    /// - Takes arguments by value.
    fn int_gcd_ext(self, other: Self::Rhs) -> Result<[Self::Out; 3]> where Self: Sized{ E::ni() }
    /// Returns the <abbr title="Greatest Common Divisor">GCD</abbr> and the Bézout coeficients.
    /// - Takes arguments by reference.
    fn int_ref_gcd_ext(&self, other: &Self::Rhs) -> Result<[Self::Out; 3]>
        where Self: Sized{ E::ni() }

    /// Returns the <abbr title="Least Common Multiple">LCM</abbr>.
    /// - Takes arguments by value.
    fn int_lcm(self, other: Self::Rhs) -> Result<Self::Out> where Self: Sized{ E::ni() }
    /// Returns the <abbr title="Least Common Multiple">LCM</abbr>.
    /// - Takes arguments by reference.
    fn int_ref_lcm(&self, other: &Self::Rhs) -> Result<Self::Out> where Self: Sized{ E::ni() }
}

/// Common trait for referenced integer types.
///
/// It is automatically implemented for references of types implementing [`NumInt`].
/// Mutable operations are only available for exclusive (`&mut`) references.
#[cfg_attr(feature = "nightly", doc(cfg(feature = "num")))]
#[rustfmt::skip] #[allow(unused_variables)]
pub trait NumRefInt<'a>: NumRef<'a>
where
    Self: Deref<Target = <Self as NumRef<'a>>::Own>,
    <Self as NumRef<'a>>::Own: NumInt
{

    /* digital root */

    /// Returns the digital root in base 10.
    fn int_ref_digital_root(&self) -> Result<<Self::Own as Num>::Out> {
            self.deref().int_ref_digital_root()
    }
    /// Returns the digital root in the given `base`.
    fn int_ref_digital_root_base(&self, base: &<Self::Own as Num>::Rhs)
        -> Result<<Self::Own as Num>::Out> {
            self.deref().int_ref_digital_root_base(base)
    }

    /* digits */

    /// Returns the number of digits in base 10.
    fn int_ref_digits(&self) -> Result<<Self::Own as Num>::Out> {
            self.deref().int_ref_digits()
    }
    /// Returns the number of digits in base 10 including the negative sign.
    fn int_ref_digits_sign(&self) -> Result<<Self::Own as Num>::Out> {
            self.deref().int_ref_digits_sign()
    }
    /// Returns the number of digits in the given `base`.
    fn int_ref_digits_base(&self, base: &<Self::Own as Num>::Rhs)
        -> Result<<Self::Own as Num>::Out> {
            self.deref().int_ref_digits_base(base)
    }
    /// Returns the number of digits in the given `base`.
    fn int_ref_digits_base_sign(&self, base: &<Self::Own as Num>::Rhs)
        -> Result<<Self::Own as Num>::Out> {
            self.deref().int_ref_digits_base_sign(base)
    }

    /* gcd & lcm */

    /// Returns the <abbr title="Greatest Common Divisor">GCD</abbr>.
    fn int_ref_gcd(&self, other: &<Self::Own as Num>::Rhs) -> Result<<Self::Own as Num>::Out> {
            self.deref().int_ref_gcd(other)
    }
    /// Returns the <abbr title="Greatest Common Divisor">GCD</abbr> and the Bézout coeficients.
    fn int_ref_gcd_ext(&self, other: &<Self::Own as Num>::Rhs)
        -> Result<[<Self::Own as Num>::Out; 3]> {
            self.deref().int_ref_gcd_ext(other)
    }
}
