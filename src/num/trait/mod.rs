// devela::num::trait
//
//!
//

use crate::num::{NumErrors as E, NumResult as Result};
use core::ops::{Deref, DerefMut};
#[cfg(doc)]
use E::{NotImplemented, NotSupported};

mod impls;

#[rustfmt::skip]
impl<'a, T: Num> NumRef<'a> for &T { type Own = T; }
#[rustfmt::skip]
impl<'a, T: Num> NumRef<'a> for &mut T { type Own = T; }

/// Common trait for numeric types.
///
/// # Notes
/// - Every method in this trait returns [`NotImplemented`] by default.
/// - Any concrete implementation must implement the operations it wants to support.
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
#[cfg_attr(feature = "nightly", doc(cfg(feature = "num")))]
#[rustfmt::skip] #[allow(unused_variables)]
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

    /// Computes `self - rhs` (subtraction).
    fn num_sub(self, rhs: Self::Rhs) -> Result<Self::Out> where Self: Sized { E::ni() }
    /// *Like [`num_sub`][Self::num_sub] but takes the arguments by reference.*
    fn num_ref_sub(&self, rhs: &Self::Rhs) -> Result<Self::Out> { E::ni() }

    /// Computes `self * rhs` (multiplication).
    fn num_mul(self, rhs: Self::Rhs) -> Result<Self::Out> where Self: Sized { E::ni() }
    /// *Like [`num_mul`][Self::num_mul] but takes the arguments by reference.*
    fn num_ref_mul(&self, rhs: &Self::Rhs) -> Result<Self::Out> { E::ni() }

    /// Computes `self / rhs` (division).
    fn num_div(self, rhs: Self::Rhs) -> Result<Self::Out> where Self: Sized { E::ni() }
    /// *Like [`num_div`][Self::num_div] but takes the arguments by reference.*
    fn num_ref_div(&self, rhs: &Self::Rhs) -> Result<Self::Out> { E::ni() }

    /// Computes `self % rhs` (remainder).
    fn num_rem(self, rhs: Self::Rhs) -> Result<Self::Out> where Self: Sized { E::ni() }
    /// *Like [`num_rem`][Self::num_rem] but takes the arguments by reference.*
    fn num_ref_rem(&self, rhs: &Self::Rhs) -> Result<Self::Out> { E::ni() }

    /// Computes `-self` (additive inverse).
    fn num_neg(self) -> Result<Self::Out> where Self: Sized { E::ni() }
    /// *Like [`num_neg`][Self::num_neg] but takes the arguments by reference.*
    fn num_ref_neg(&self) -> Result<Self::Out> { E::ni() }

    /// Computes `|self|` (absolute value).
    fn num_abs(self) -> Result<Self::Out> where Self: Sized { E::ni() }
    /// *Like [`num_abs`][Self::num_abs] but takes the arguments by reference.*
    fn num_ref_abs(&self) -> Result<Self::Out> { E::ni() }
}

/// Common trait for referenced numeric types.
///
/// It is automatically implemented for references of types implementing [`Num`].
/// Mutable operations are only available for exclusive (`&mut`) references.
#[cfg_attr(feature = "nightly", doc(cfg(feature = "num")))]
#[rustfmt::skip] #[allow(unused_variables)]
pub trait NumRef<'a>
where
    Self: Deref<Target = Self::Own>,
{
    /// The owned version of this numeric type.
    type Own: Num;

    /// Returns the owned version of `self`, if it can be cloned.
    fn num_to_owned(&self) -> Result<Self::Own> where Self::Own: Clone { Ok((*self).clone()) }

    /// Sets `self` to a valid `value`.
    fn num_set(&mut self, value: <Self::Own as Num>::Inner) -> Result<()>
        where Self: DerefMut<Target = Self::Own> { self.deref_mut().num_set(value) }

    /// Sets `self` to a valid `&value`.
    fn num_set_ref(&mut self, value: &<Self::Own as Num>::Inner) -> Result<()>
        where Self: DerefMut<Target = Self::Own> { self.deref_mut().num_set_ref(value) }

    /* Identities */

    /// Returns `true` if `self` is zero.
    fn num_is_zero(&self) -> Result<bool> { self.deref().num_is_zero() }
    /// Returns the number zero.
    fn num_get_zero() -> Result<Self::Own> { Self::Own::num_get_zero() }
    /// Sets `self` to zero.
    fn num_set_zero(&mut self) -> Result<()>
        where Self: DerefMut<Target = Self::Own> { self.deref_mut().num_set_zero() }

    /// Returns `true` if `self` is one.
    fn num_is_one(&self) -> Result<bool> { self.deref().num_is_one() }
    /// Returns the number one.
    fn num_get_one() -> Result<Self::Own> { Self::Own::num_get_one() }
    /// Sets the number to one.
    fn num_set_one(&mut self) -> Result<()>
        where Self: DerefMut<Target = Self::Own> { self.deref_mut().num_set_one() }

    /* Operations */

    /// Computes `&self + &rhs`.
    fn num_ref_add(&self, rhs: &<Self::Own as Num>::Rhs) -> Result<<Self::Own as Num>::Out> {
        self.deref().num_ref_add(rhs) }

    /// Computes `&self - &rhs`.
    fn num_ref_sub(&self, rhs: &<Self::Own as Num>::Rhs) -> Result<<Self::Own as Num>::Out> {
        self.deref().num_ref_sub(rhs) }

    /// Computes `&self * &rhs`.
    fn num_ref_mul(&self, rhs: &<Self::Own as Num>::Rhs) -> Result<<Self::Own as Num>::Out> {
        self.deref().num_ref_mul(rhs) }

    /// Computes `&self / &rhs`.
    fn num_ref_div(&self, rhs: &<Self::Own as Num>::Rhs) -> Result<<Self::Own as Num>::Out> {
        self.deref().num_ref_div(rhs) }

    /// Computes `&self % &rhs`.
    fn num_ref_rem(&self, rhs: &<Self::Own as Num>::Rhs) -> Result<<Self::Own as Num>::Out> {
        self.deref().num_ref_rem(rhs) }

    /// Computes `- &self`.
    fn num_ref_neg(&self) -> Result<<Self::Own as Num>::Out> { self.deref().num_ref_neg() }

    /// Computes the absolute value of `&self`.
    fn num_ref_abs(&self) -> Result<<Self::Own as Num>::Out> { self.deref().num_ref_abs() }
}
