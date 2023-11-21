// devela::math::num::traits::num
//
//!
//

use crate::math::num::{NumError as Error, NumResult as Result};
use core::ops::{Deref, DerefMut};

mod impls;
// #[cfg(feature = "sync")]
// mod sync_impls;
mod auto_impls {
    use super::{NoNum, Num, NumRef};

    #[rustfmt::skip]
    impl Num for NoNum {
        type Inner = ();
        type Out = ();
        type Rhs = ();

        fn num_into(self) -> Self::Inner {}
    }

    #[rustfmt::skip]
    impl<'a, T: Num> NumRef<'a> for &T { type Own = T; }

    #[rustfmt::skip]
    impl<'a, T: Num> NumRef<'a> for &mut T { type Own = T; }
}

/// Represents the absence of a number.
///
/// This can be used anywhere an implementation of [`Num`] is expected,
/// because it implements all the numeric traits, while doing nothing.
pub type NoNum = ();

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
/// See also [`NumRef`] that is intended to be implemented for `Num` references.
#[rustfmt::skip]
#[allow(unused_variables)]
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
        where Self: Sized { Error::notimpl() }
    /// Returns `Self` if given a valid `&value`.
    fn num_from_ref(value: &Self::Inner) -> Result<Self>
        where Self: Sized { Error::notimpl() }

    /// Sets `self` to the given valid `value`.
    fn num_set(&mut self, value: Self::Inner) -> Result<()> { Error::notimpl() }
    /// Sets `self` to the given valid `&value`.
    fn num_set_ref(&mut self, value: &Self::Inner) -> Result<()> { Error::notimpl() }

    /* Identities */

    /// Returns `true` if `self` is zero.
    fn num_is_zero(&self) -> Result<bool> { Error::notimpl() }
    /// Returns the number zero.
    fn num_get_zero() -> Result<Self>
        where Self: Sized { Error::notimpl() }
    /// Sets `self` to `0`.
    fn num_set_zero(&mut self) -> Result<()> { Error::notimpl() }

    /// Returns `true` if `self` is one.
    fn num_is_one(&self) -> Result<bool> { Error::notimpl() }
    /// Returns the number one.
    fn num_get_one() -> Result<Self>
        where Self: Sized { Error::notimpl() }
    /// Sets the number to one.
    fn num_set_one(&mut self) -> Result<()> { Error::notimpl() }

    /* Operations */

    /// Computes `self` + `other`.
    fn num_add(self, rhs: Self::Rhs) -> Result<Self::Out>
        where Self: Sized { Error::notimpl() }
    /// Computes `self` + `&other`.
    fn num_add_ref(self, rhs: &Self::Rhs) -> Result<Self::Out>
        where Self: Sized { Error::notimpl() }
    /// Computes `&self` + `other`.
    fn num_ref_add(&self, rhs: Self::Rhs) -> Result<Self::Out>
        where Self: Sized { Error::notimpl() }
    /// Computes `&self` + `&other`.
    fn num_ref_add_ref(&self, rhs: &Self::Rhs) -> Result<Self::Out>
        where Self: Sized { Error::notimpl() }

    /// Computes `self` - `other`.
    fn num_sub(self, rhs: Self::Rhs) -> Result<Self::Out>
        where Self: Sized { Error::notimpl() }
    /// Computes `self` - `&other`.
    fn num_sub_ref(self, rhs: &Self::Rhs) -> Result<Self::Out>
        where Self: Sized { Error::notimpl() }
    /// Computes `&self` - `other`.
    fn num_ref_sub(&self, rhs: Self::Rhs) -> Result<Self::Out> 
        where Self: Sized { Error::notimpl() }
    /// Computes `&self` - `&other`.
    fn num_ref_sub_ref(&self, rhs: &Self::Rhs) -> Result<Self::Out>
        where Self: Sized { Error::notimpl() }

    /// Computes `self` * `other`.
    fn num_mul(self, rhs: Self::Rhs) -> Result<Self::Out>
        where Self: Sized { Error::notimpl() }
    /// Computes `self` * `&other`.
    fn num_mul_ref(self, rhs: &Self::Rhs) -> Result<Self::Out>
        where Self: Sized { Error::notimpl() }
    /// Computes `&self` * `other`.
    fn num_ref_mul(&self, rhs: Self::Rhs) -> Result<Self::Out>
        where Self: Sized { Error::notimpl() }
    /// Computes `&self` * `&other`.
    fn num_ref_mul_ref(&self, rhs: &Self::Rhs) -> Result<Self::Out>
        where Self: Sized { Error::notimpl() }

    /// Computes `self` / `other`.
    fn num_div(self, rhs: Self::Rhs) -> Result<Self::Out>
        where Self: Sized { Error::notimpl() }
    /// Computes `self` / `&other`.
    fn num_div_ref(self, rhs: &Self::Rhs) -> Result<Self::Out>
        where Self: Sized { Error::notimpl() }
    /// Computes `&self` / `other`.
    fn num_ref_div(&self, rhs: Self::Rhs) -> Result<Self::Out>
        where Self: Sized { Error::notimpl() }
    /// Computes `&self` / `&other`.
    fn num_ref_div_ref(&self, rhs: &Self::Rhs) -> Result<Self::Out>
        where Self: Sized { Error::notimpl() }

    /// Computes `self` % `other`.
    fn num_rem(self, rhs: Self::Rhs) -> Result<Self::Out>
        where Self: Sized { Error::notimpl() }
    /// Computes `self` % `&other`.
    fn num_rem_ref(self, rhs: &Self::Rhs) -> Result<Self::Out>
        where Self: Sized { Error::notimpl() }
    /// Computes `&self` % `other`.
    fn num_ref_rem(&self, rhs: Self::Rhs) -> Result<Self::Out>
        where Self: Sized { Error::notimpl() }
    /// Computes `&self` % `&other`.
    fn num_ref_rem_ref(&self, rhs: &Self::Rhs) -> Result<Self::Out>
        where Self: Sized { Error::notimpl() }

    /// Computes `- self`.
    fn num_neg(self) -> Result<Self::Out>
        where Self: Sized { Error::notimpl() }
    /// Computes `- &self`.
    fn num_ref_neg(&self) -> Result<Self::Out>
        where Self: Sized { Error::notimpl() }

    /// Computes the absolute value of `self`.
    fn num_abs(self) -> Result<Self::Out>
        where Self: Sized { Error::notimpl() }
    /// Computes the absolute value of `&self`.
    fn num_ref_abs(&self) -> Result<Self::Out>
        where Self: Sized { Error::notimpl() }
}

/// Common trait for referenced numeric types.
///
/// Not that mutable operations can only be implemented when `NumRef` is
/// implemented for exclusive references (&mut).
///
/// See also [`Num`] which is intended to be implemented for the owned type.
#[rustfmt::skip]
#[allow(unused_variables)]
pub trait NumRef<'a> where Self: Deref<Target = Self::Own> {
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

    /// Computes `&self` + `rhs`.
    fn num_ref_add(&self, rhs: <Self::Own as Num>::Rhs)
        -> Result<<Self::Own as Num>::Out> {
        self.deref().num_ref_add(rhs)
    }
    /// Computes `&self` + `&rhs`.
    fn num_ref_add_ref(&self, rhs: &<Self::Own as Num>::Rhs)
        -> Result<<Self::Own as Num>::Out> {
        self.deref().num_ref_add_ref(rhs)
    }

    /// Computes `&self` - `rhs`.
    fn num_ref_sub(&self, rhs: <Self::Own as Num>::Rhs)
        -> Result<<Self::Own as Num>::Out> {
        self.deref().num_ref_sub(rhs)
    }
    /// Computes `&self` - `&rhs`.
    fn num_ref_sub_ref(&self, rhs: &<Self::Own as Num>::Rhs)
        -> Result<<Self::Own as Num>::Out> {
        self.deref().num_ref_sub_ref(rhs)
    }

    /// Computes `&self` * `rhs`.
    fn num_ref_mul(&self, rhs: <Self::Own as Num>::Rhs)
        -> Result<<Self::Own as Num>::Out> {
        self.deref().num_ref_mul(rhs)
    }
    /// Computes `&self` * `&rhs`.
    fn num_ref_mul_ref(&self, rhs: &<Self::Own as Num>::Rhs)
        -> Result<<Self::Own as Num>::Out> {
        self.deref().num_ref_mul_ref(rhs)
    }

    /// Computes `&self` / `rhs`.
    fn num_ref_div(&self, rhs: <Self::Own as Num>::Rhs)
        -> Result<<Self::Own as Num>::Out> {
        self.deref().num_ref_div(rhs)
    }
    /// Computes `&self` / `&rhs`.
    fn num_ref_div_ref(&self, rhs: &<Self::Own as Num>::Rhs)
        -> Result<<Self::Own as Num>::Out> {
        self.deref().num_ref_div_ref(rhs)
    }

    /// Computes `&self` % `rhs`.
    fn num_ref_rem(&self, rhs: <Self::Own as Num>::Rhs)
        -> Result<<Self::Own as Num>::Out> {
        self.deref().num_ref_rem(rhs)
    }
    /// Computes `&self` % `&rhs`.
    fn num_ref_rem_ref(&self, rhs: &<Self::Own as Num>::Rhs)
        -> Result<<Self::Own as Num>::Out> {
        self.deref().num_ref_rem_ref(rhs)
    }

    /// Computes `- &self`.
    fn num_ref_neg(&self) -> Result<<Self::Own as Num>::Out> { self.deref().num_ref_neg() }

    /// Computes the absolute value of `&self`.
    fn num_ref_abs(&self) -> Result<<Self::Own as Num>::Out> { self.deref().num_ref_abs() }
}
