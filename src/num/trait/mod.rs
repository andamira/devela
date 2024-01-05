// devela::num::trait
//
//!
//

use crate::num::{NumErrors as Error, NumResult as Result};
use core::ops::{Deref, DerefMut};

mod impls;

#[rustfmt::skip]
impl<'a, T: Num> NumRef<'a> for &T { type Own = T; }
#[rustfmt::skip]
impl<'a, T: Num> NumRef<'a> for &mut T { type Own = T; }

/// Common trait for numeric types.
///
/// This trait doesn't depend on having any operations implemented, and it
/// offers a common numeric API that returns [`NotImplemented`][Error::NotImplemented]
/// by default unless the methods are overriden.
///
/// Any concrete implementation must manually implement the operations it wants.
///
/// You could also ask for additional bounds like e.g. [`Add`][core::ops::Add].
///
/// Most methods come in two variants, starting with different prefixes:
/// and the other one takes the arguments by reference.
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
/// See also [`NumRef`] that is automatically implemented for `Num` references.
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
    /// - Takes arguments by value.
    fn num_from(value: Self::Inner) -> Result<Self>
        where Self: Sized { Error::ni() }
    /// Returns `Self` if given a valid `&value`.
    /// - Takes arguments by reference.
    fn num_from_ref(value: &Self::Inner) -> Result<Self>
        where Self: Sized { Error::ni() }

    /// Sets `self` to the given valid `value`.
    /// - Takes arguments by value.
    fn num_set(&mut self, value: Self::Inner) -> Result<()> { Error::ni() }
    /// Sets `self` to the given valid `&value`.
    /// - Takes arguments by reference.
    fn num_set_ref(&mut self, value: &Self::Inner) -> Result<()> { Error::ni() }

    /* Identities */

    /// Returns `true` if `self` is zero.
    fn num_is_zero(&self) -> Result<bool> { Error::ni() }
    /// Returns the number zero.
    fn num_get_zero() -> Result<Self>
        where Self: Sized { Error::ni() }
    /// Sets `self` to `0`.
    /// - Takes arguments by reference.
    fn num_set_zero(&mut self) -> Result<()> { Error::ni() }

    /// Returns `true` if `self` is one.
    /// - Takes arguments by reference.
    fn num_is_one(&self) -> Result<bool> { Error::ni() }
    /// Returns the number one.
    fn num_get_one() -> Result<Self>
        where Self: Sized { Error::ni() }
    /// Sets the number to one.
    /// - Takes arguments by reference.
    fn num_set_one(&mut self) -> Result<()> { Error::ni() }

    /* Operations */

    /// Computes `self + other`.
    /// - Takes arguments by value.
    fn num_add(self, rhs: Self::Rhs) -> Result<Self::Out>
        where Self: Sized { Error::ni() }
    /// Computes `&self + &other`.
    /// - Takes arguments by reference.
    fn num_ref_add(&self, rhs: &Self::Rhs) -> Result<Self::Out>
        where Self: Sized { Error::ni() }

    /// Computes `self - other`.
    /// - Takes arguments by value.
    fn num_sub(self, rhs: Self::Rhs) -> Result<Self::Out>
        where Self: Sized { Error::ni() }
    /// Computes `&self - &other`.
    /// - Takes arguments by reference.
    fn num_ref_sub(&self, rhs: &Self::Rhs) -> Result<Self::Out>
        where Self: Sized { Error::ni() }

    /// Computes `self * other`.
    /// - Takes arguments by value.
    fn num_mul(self, rhs: Self::Rhs) -> Result<Self::Out>
        where Self: Sized { Error::ni() }
    /// Computes `&self * &other`.
    /// - Takes arguments by reference.
    fn num_ref_mul(&self, rhs: &Self::Rhs) -> Result<Self::Out>
        where Self: Sized { Error::ni() }

    /// Computes `self / other`.
    /// - Takes arguments by value.
    fn num_div(self, rhs: Self::Rhs) -> Result<Self::Out>
        where Self: Sized { Error::ni() }
    /// Computes `&self / &other`.
    /// - Takes arguments by reference.
    fn num_ref_div(&self, rhs: &Self::Rhs) -> Result<Self::Out>
        where Self: Sized { Error::ni() }

    /// Computes `self % other`.
    /// - Takes arguments by value.
    fn num_rem(self, rhs: Self::Rhs) -> Result<Self::Out>
        where Self: Sized { Error::ni() }
    /// Computes `&self % &other`.
    /// - Takes arguments by reference.
    fn num_ref_rem(&self, rhs: &Self::Rhs) -> Result<Self::Out>
        where Self: Sized { Error::ni() }

    /// Computes `- self`.
    /// - Takes arguments by value.
    fn num_neg(self) -> Result<Self::Out>
        where Self: Sized { Error::ni() }
    /// Computes `- &self`.
    /// - Takes arguments by reference.
    fn num_ref_neg(&self) -> Result<Self::Out>
        where Self: Sized { Error::ni() }

    /// Computes the absolute value of `self`.
    /// - Takes arguments by value.
    fn num_abs(self) -> Result<Self::Out>
        where Self: Sized { Error::ni() }
    /// Computes the absolute value of `&self`.
    /// - Takes arguments by reference.
    fn num_ref_abs(&self) -> Result<Self::Out>
        where Self: Sized { Error::ni() }
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
    fn num_to_owned(&self) -> Result<Self::Own>
        where Self::Own: Clone { Ok((*self).clone()) }

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
