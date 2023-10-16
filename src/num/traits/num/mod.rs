// devela::num::traits::num
//
//!
//

use core::ops::{Deref, DerefMut};

mod impls;

/// Represents the absence of a number.
///
/// This can be used anywhere an implementation of [`Num`] is expected,
/// because it implements all the numeric traits, while doing nothing.
pub type NoNum = ();
#[rustfmt::skip]
impl Num for NoNum { type Inner = (); fn num_into(self) -> Self::Inner {} }

/// Common trait for numeric types.
///
/// This trait doesn't depend on having any operations implemented, and it
/// offers a common numeric API that returns `None` by default unless the
/// methods are overriden.
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
/// If there's an error or an impossibility at any point it just returns `None`.
///
/// The standard library offers different methods for signed and unsigned types,
/// (e.g. abs, neg), and some are lacking for non-zero types (div, sub).
/// This trait try to offer the same methods everywhere and give a
/// result when a result is possible.
///
/// See also [`NumRef`] that is intended to be implemented for `Num` references.
#[rustfmt::skip]
#[allow(unused_variables)]
pub trait Num: PartialEq {
    /// The internal representation of this numeric type.
    type Inner;

    /// Returns the inner `self` representation.
    #[must_use]
    fn num_into(self) -> Self::Inner;

    /// Returns `Self` when given a valid `value`.
    #[must_use]
    fn num_from(value: Self::Inner) -> Option<Self> where Self: Sized { None }
    /// Returns `Self` when given a valid `&value`.
    #[must_use]
    fn num_from_ref(value: &Self::Inner) -> Option<Self> where Self: Sized { None }

    /// Sets `self` to the given valid `value`.
    #[must_use]
    fn num_set(&mut self, value: Self::Inner) -> Option<()> { None }
    /// Sets `self` to the given valid `&value`.
    #[must_use]
    fn num_set_ref(&mut self, value: &Self::Inner) -> Option<()> { None }

    /* Identities */

    /// Returns `true` if `self` is zero.
    #[must_use]
    fn num_is_zero(&self) -> Option<bool> { None }
    /// Returns the number zero.
    #[must_use]
    fn num_get_zero() -> Option<Self> where Self:Sized { None }
    /// Sets `self` to `0`.
    #[must_use]
    fn num_set_zero(&mut self) -> Option<()> { None }

    /// Returns `true` if `self` is one.
    #[must_use]
    fn num_is_one(&self) -> Option<bool> { None }
    /// Returns the number one.
    #[must_use]
    fn num_get_one() -> Option<Self> where Self:Sized { None }
    /// Sets the number to one.
    #[must_use]
    fn num_set_one(&mut self) -> Option<()> { None }

    /* Operations */

    /// Computes `self` + `other`.
    #[must_use]
    fn num_add(self, other: Self) -> Option<Self> where Self: Sized { None }
    /// Computes `self` + `&other`.
    #[must_use]
    fn num_add_ref(self, other: &Self) -> Option<Self> where Self:Sized { None }
    /// Computes `&self` + `other`.
    #[must_use]
    fn num_ref_add(&self, other: Self) -> Option<Self> where Self: Sized { None }
    /// Computes `&self` + `&other`.
    #[must_use]
    fn num_ref_add_ref(&self, other: &Self) -> Option<Self> where Self:Sized { None }

    /// Computes `self` - `other`.
    #[must_use]
    fn num_sub(self, other: Self) -> Option<Self> where Self: Sized { None }
    /// Computes `self` - `&other`.
    #[must_use]
    fn num_sub_ref(self, other: &Self) -> Option<Self> where Self: Sized { None }
    /// Computes `&self` - `other`.
    #[must_use]
    fn num_ref_sub(&self, other: Self) -> Option<Self> where Self: Sized { None }
    /// Computes `&self` - `&other`.
    #[must_use]
    fn num_ref_sub_ref(&self, other: &Self) -> Option<Self> where Self:Sized { None }

    /// Computes `self` * `other`.
    #[must_use]
    fn num_mul(self, other: Self) -> Option<Self> where Self: Sized { None }
    /// Computes `self` * `&other`.
    #[must_use]
    fn num_mul_ref(self, other: &Self) -> Option<Self> where Self:Sized { None }
    /// Computes `&self` * `other`.
    #[must_use]
    fn num_ref_mul(&self, other: Self) -> Option<Self> where Self: Sized { None }
    /// Computes `&self` * `&other`.
    #[must_use]
    fn num_ref_mul_ref(&self, other: &Self) -> Option<Self> where Self:Sized { None }

    /// Computes `self` / `other`.
    #[must_use]
    fn num_div(self, other: Self) -> Option<Self> where Self: Sized { None }
    /// Computes `self` / `&other`.
    #[must_use]
    fn num_div_ref(self, other: &Self) -> Option<Self> where Self:Sized { None }
    /// Computes `&self` / `other`.
    #[must_use]
    fn num_ref_div(&self, other: Self) -> Option<Self> where Self: Sized { None }
    /// Computes `&self` / `&other`.
    #[must_use]
    fn num_ref_div_ref(&self, other: &Self) -> Option<Self> where Self:Sized { None }

    /// Computes `self` % `other`.
    #[must_use]
    fn num_rem(self, other: Self) -> Option<Self> where Self: Sized { None }
    /// Computes `self` % `&other`.
    #[must_use]
    fn num_rem_ref(self, other: &Self) -> Option<Self> where Self:Sized { None }
    /// Computes `&self` % `other`.
    #[must_use]
    fn num_ref_rem(&self, other: Self) -> Option<Self> where Self: Sized { None }
    /// Computes `&self` % `&other`.
    #[must_use]
    fn num_ref_rem_ref(&self, other: &Self) -> Option<Self> where Self:Sized { None }

    /// Computes `- self`.
    #[must_use]
    fn num_neg(self) -> Option<Self> where Self:Sized { None }
    /// Computes `- &self`.
    #[must_use]
    fn num_ref_neg(&self) -> Option<Self> where Self:Sized { None }

    /// Computes the absolute value of `self`.
    #[must_use]
    fn num_abs(self) -> Option<Self> where Self: Sized { None }
    /// Computes the absolute value of `&self`.
    #[must_use]
    fn num_ref_abs(&self) -> Option<Self> where Self: Sized { None }
}

impl<'a, T: Num> NumRef<'a> for &T {
    type Own = T;
}
impl<'a, T: Num> NumRef<'a> for &mut T {
    type Own = T;
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
    #[must_use]
    fn num_to_owned(&self) -> Option<Self::Own> where Self::Own: Clone { Some((*self).clone()) }

    /// Sets `self` to a valid `value`.
    fn num_set(&mut self, value: <Self::Own as Num>::Inner) -> Option<()>
        where Self: DerefMut<Target = Self::Own> { self.deref_mut().num_set(value) }

    /// Sets `self` to a valid `&value`.
    fn num_set_ref(&mut self, value: &<Self::Own as Num>::Inner) -> Option<()>
        where Self: DerefMut<Target = Self::Own> { self.deref_mut().num_set_ref(value) }

    /* Identities */

    /// Returns `true` if `self` is zero.
    #[must_use]
    fn num_is_zero(&self) -> Option<bool> { self.deref().num_is_zero() }
    /// Returns the number zero.
    #[must_use]
    fn num_get_zero() -> Option<Self::Own> { Self::Own::num_get_zero() }
    /// Sets `self` to zero.
    #[must_use]
    fn num_set_zero(&mut self) -> Option<()>
        where Self: DerefMut<Target = Self::Own> { self.deref_mut().num_set_zero() }

    /// Returns `true` if `self` is one.
    #[must_use]
    fn num_is_one(&self) -> Option<bool> { self.deref().num_is_one() }
    /// Returns the number one.
    #[must_use]
    fn num_get_one() -> Option<Self::Own> { Self::Own::num_get_one() }
    /// Sets the number to one.
    #[must_use]
    fn num_set_one(&mut self) -> Option<()>
        where Self: DerefMut<Target = Self::Own> { self.deref_mut().num_set_one() }

    /* Operations */

    /// Computes `&self` + `other`.
    #[must_use]
    fn num_ref_add(&self, other: Self::Own) -> Option<Self::Own> {
        self.deref().num_ref_add(other)
    }
    /// Computes `&self` + `&other`.
    #[must_use]
    fn num_ref_add_ref(&self, other: &Self::Own) -> Option<Self::Own> {
        self.deref().num_ref_add_ref(other)
    }

    /// Computes `&self` - `other`.
    #[must_use]
    fn num_ref_sub(&self, other: Self::Own) -> Option<Self::Own> {
        self.deref().num_ref_sub(other)
    }
    /// Computes `&self` - `&other`.
    #[must_use]
    fn num_ref_sub_ref(&self, other: &Self::Own) -> Option<Self::Own> {
        self.deref().num_ref_sub_ref(other)
    }

    /// Computes `&self` * `other`.
    #[must_use]
    fn num_ref_mul(&self, other: Self::Own) -> Option<Self::Own> {
        self.deref().num_ref_mul(other)
    }
    /// Computes `&self` * `&other`.
    #[must_use]
    fn num_ref_mul_ref(&self, other: &Self::Own) -> Option<Self::Own> {
        self.deref().num_ref_mul_ref(other)
    }

    /// Computes `&self` / `other`.
    #[must_use]
    fn num_ref_div(&self, other: Self::Own) -> Option<Self::Own> {
        self.deref().num_ref_div(other)
    }
    /// Computes `&self` / `&other`.
    #[must_use]
    fn num_ref_div_ref(&self, other: &Self::Own) -> Option<Self::Own> {
        self.deref().num_ref_div_ref(other)
    }

    /// Computes `&self` % `other`.
    #[must_use]
    fn num_ref_rem(&self, other: Self::Own) -> Option<Self::Own> {
        self.deref().num_ref_rem(other)
    }
    /// Computes `&self` % `&other`.
    #[must_use]
    fn num_ref_rem_ref(&self, other: &Self::Own) -> Option<Self::Own> {
        self.deref().num_ref_rem_ref(other)
    }

    /// Computes `- &self`.
    #[must_use]
    fn num_ref_neg(&self) -> Option<Self::Own> { self.deref().num_ref_neg() }

    /// Computes the absolute value of `&self`.
    #[must_use]
    fn num_ref_abs(&self) -> Option<Self::Own> { self.deref().num_ref_abs() }
}
