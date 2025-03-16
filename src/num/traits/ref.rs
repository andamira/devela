// devela::num::traits::ref
//
//! Defines the [`NumRef`] trait.
//

use crate::{Deref, DerefMut, Num, NumResult as Result};

#[doc = crate::TAG_NUM!()]
/// Common auto-trait for referenced numeric types.
///
/// It is automatically implemented for references of types implementing [`Num`].
/// Mutable operations are only available for exclusive (`&mut`) references.
#[rustfmt::skip]
#[cfg_attr(nightly_doc, doc(notable_trait))]
pub trait NumRef<'a> where Self: Deref<Target = Self::Own> {
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
    /// Computes `&mut self += &rhs`.
    fn num_ref_add_assign(&mut self, rhs: &<Self::Own as Num>::Rhs) -> Result<()>
    where Self: DerefMut<Target = Self::Own> { self.deref_mut().num_ref_add_assign(rhs) }

    /// Computes `&self - &rhs`.
    fn num_ref_sub(&self, rhs: &<Self::Own as Num>::Rhs) -> Result<<Self::Own as Num>::Out> {
        self.deref().num_ref_sub(rhs) }
    /// Computes `&mut self -= &rhs`.
    fn num_ref_sub_assign(&mut self, rhs: &<Self::Own as Num>::Rhs) -> Result<()>
    where Self: DerefMut<Target = Self::Own> { self.deref_mut().num_ref_sub_assign(rhs) }

    /// Computes `&self * &rhs`.
    fn num_ref_mul(&self, rhs: &<Self::Own as Num>::Rhs) -> Result<<Self::Own as Num>::Out> {
        self.deref().num_ref_mul(rhs) }
    /// Computes `&mut self *= &rhs`.
    fn num_ref_mul_assign(&mut self, rhs: &<Self::Own as Num>::Rhs) -> Result<()>
    where Self: DerefMut<Target = Self::Own> { self.deref_mut().num_ref_mul_assign(rhs) }

    /// Computes `&self / &rhs`.
    fn num_ref_div(&self, rhs: &<Self::Own as Num>::Rhs) -> Result<<Self::Own as Num>::Out> {
        self.deref().num_ref_div(rhs) }
    /// Computes `&mut self /= &rhs`.
    fn num_ref_div_assign(&mut self, rhs: &<Self::Own as Num>::Rhs) -> Result<()>
    where Self: DerefMut<Target = Self::Own> { self.deref_mut().num_ref_div_assign(rhs) }

    /// Computes `&self % &rhs`.
    fn num_ref_rem(&self, rhs: &<Self::Own as Num>::Rhs) -> Result<<Self::Own as Num>::Out> {
        self.deref().num_ref_rem(rhs) }
    /// Computes `&mut self %= &rhs`.
    fn num_ref_rem_assign(&mut self, rhs: &<Self::Own as Num>::Rhs) -> Result<()>
    where Self: DerefMut<Target = Self::Own> { self.deref_mut().num_ref_rem_assign(rhs) }

    /// Computes `- &self`.
    fn num_ref_neg(&self) -> Result<<Self::Own as Num>::Out> { self.deref().num_ref_neg() }

    /// Computes the absolute value of `&self`.
    fn num_ref_abs(&self) -> Result<<Self::Own as Num>::Out> { self.deref().num_ref_abs() }
}
