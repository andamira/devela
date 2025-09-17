// devela::num::traits
//
//!
//

use crate::{NumError as E, NumResult as Result, sf};
#[cfg(doc)]
use E::{NotImplemented, NotSupported};

mod constants; // NumConst
mod r#ref; // NumRef
pub use {constants::*, r#ref::*};

mod impls;

sf! {
    impl<T: Num> NumRef<'_> for &T { type Own = T; }
    impl<T: Num> NumRef<'_> for &mut T { type Own = T; }
}

#[doc = crate::_TAG_NUM!()]
/// Common trait for numeric types.
///
/// # Notes
/// - Every method has a default implementation that returns [`NotImplemented`].
/// - Specific implementations can customize the operations it wants to support.
/// - Any operations specifically not supported should ideally return [`NotSupported`].
///
/// - Most methods come in two variants, starting with different prefixes:
///   - `num_*` methods take the arguments by value.
///   - `num_ref_*` methods take the arguments by reference.
///
/// - This trait tries to offer the same methods everywhere, giving a result when possible.
/// - Operations are generally supported if they can be valid for some values.
/// E.g. `num_abs` for unsigned types is only valid for `0`.
///
/// See also [`NumRef`] which is automatically implemented for `Num` references.
#[rustfmt::skip]
#[cfg_attr(nightly_doc, doc(notable_trait))]
#[allow(unused_variables, reason = "default implementation is not implemented")]
pub trait Num {
    /// The internal representation of this numeric type.
    type Inner;

    /// The output type for operations.
    type Out;

    /// The right hand side type for operations.
    type Rhs;

    /// Returns the inner `self` representation.
    #[must_use]
    fn num_into(self) -> Self::Inner;

    /// Returns `Self` if given a valid `value`.
    fn num_from(value: Self::Inner) -> Result<Self>
        where Self: Sized { E::ni() }
    /// Returns `Self` if given a valid `&value`.
    fn num_from_ref(value: &Self::Inner) -> Result<Self>
        where Self: Sized { E::ni() }

    /// Sets `self` to the given valid `value`.
    fn num_set(&mut self, value: Self::Inner) -> Result<()> { E::ni() }
    /// Sets `self` to the given valid `&value`.
    fn num_set_ref(&mut self, value: &Self::Inner) -> Result<()> { E::ni() }

    /* Identities */

    /// Returns `true` if `self` is zero.
    fn num_is_zero(&self) -> Result<bool> { E::ni() }
    /// Returns the number zero.
    fn num_get_zero() -> Result<Self> where Self: Sized { E::ni() }
    /// Sets `self` to `0`.
    fn num_set_zero(&mut self) -> Result<()> { E::ni() }

    /// Returns `true` if `self` is one.
    fn num_is_one(&self) -> Result<bool> { E::ni() }
    /// Returns the number one.
    fn num_get_one() -> Result<Self> where Self: Sized { E::ni() }
    /// Sets the number to one.
    fn num_set_one(&mut self) -> Result<()> { E::ni() }

    /* Operations */

    /// Computes `self + rhs` (addition).
    fn num_add(self, rhs: Self::Rhs) -> Result<Self::Out> where Self: Sized { E::ni() }
    /// *Like [`num_add`][Self::num_add] but takes the arguments by reference.*
    fn num_ref_add(&self, rhs: &Self::Rhs) -> Result<Self::Out> { E::ni() }
    /// Computes `&mut self += rhs;` (addition).
    fn num_ref_add_assign(&mut self, rhs: &Self::Rhs) -> Result<()> { E::ni() }

    /// Computes `self - rhs` (subtraction).
    fn num_sub(self, rhs: Self::Rhs) -> Result<Self::Out> where Self: Sized { E::ni() }
    /// *Like [`num_sub`][Self::num_sub] but takes the arguments by reference.*
    fn num_ref_sub(&self, rhs: &Self::Rhs) -> Result<Self::Out> { E::ni() }
    /// Computes `&mut self -= rhs;` (subtraction).
    fn num_ref_sub_assign(&mut self, rhs: &Self::Rhs) -> Result<()> { E::ni() }

    /// Computes `self * rhs` (multiplication).
    fn num_mul(self, rhs: Self::Rhs) -> Result<Self::Out> where Self: Sized { E::ni() }
    /// *Like [`num_mul`][Self::num_mul] but takes the arguments by reference.*
    fn num_ref_mul(&self, rhs: &Self::Rhs) -> Result<Self::Out> { E::ni() }
    /// Computes `&mut self *= rhs;` (multiplication).
    fn num_ref_mul_assign(&mut self, rhs: &Self::Rhs) -> Result<()> { E::ni() }

    /// Computes `self / rhs` (division).
    fn num_div(self, rhs: Self::Rhs) -> Result<Self::Out> where Self: Sized { E::ni() }
    /// *Like [`num_div`][Self::num_div] but takes the arguments by reference.*
    fn num_ref_div(&self, rhs: &Self::Rhs) -> Result<Self::Out> { E::ni() }
    /// Computes `&mut self /= rhs;` (division).
    fn num_ref_div_assign(&mut self, rhs: &Self::Rhs) -> Result<()> { E::ni() }

    /// Computes `self % rhs` (remainder).
    fn num_rem(self, rhs: Self::Rhs) -> Result<Self::Out> where Self: Sized { E::ni() }
    /// *Like [`num_rem`][Self::num_rem] but takes the arguments by reference.*
    fn num_ref_rem(&self, rhs: &Self::Rhs) -> Result<Self::Out> { E::ni() }
    /// Computes `&mut self %= rhs;` (remainder).
    fn num_ref_rem_assign(&mut self, rhs: &Self::Rhs) -> Result<()> { E::ni() }

    /// Computes `-self` (additive inverse).
    fn num_neg(self) -> Result<Self::Out> where Self: Sized { E::ni() }
    /// *Like [`num_neg`][Self::num_neg] but takes the arguments by reference.*
    fn num_ref_neg(&self) -> Result<Self::Out> { E::ni() }

    /// Computes `|self|` (absolute value).
    fn num_abs(self) -> Result<Self::Out> where Self: Sized { E::ni() }
    /// *Like [`num_abs`][Self::num_abs] but takes the arguments by reference.*
    fn num_ref_abs(&self) -> Result<Self::Out> { E::ni() }
}
