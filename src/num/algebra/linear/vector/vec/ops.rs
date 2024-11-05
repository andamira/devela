// devela::num::algebra::linear::vector::vec::ops
//
//! implement overloadable operators
//

#![allow(clippy::needless_range_loop)]

use crate::{
    data::Vec,
    num::{NumError::MismatchedSizes, NumResult as Result, VecVector},
};
use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

/* ops between vectors */

impl<T: Clone + Add<Output = T>> VecVector<T> {
    /// Adds two vectors together
    /// # Errors
    /// Returns [`MismatchedSizes`] if `self and `other` don't have the same length.
    pub fn clone_add(&self, other: &Self) -> Result<Self> {
        if self.coords.len() != other.coords.len() {
            Err(MismatchedSizes)
        } else {
            let mut coords = Vec::with_capacity(self.coords.len());
            for i in 0..self.coords.len() {
                coords.push(self.coords[i].clone() + other.coords[i].clone());
            }
            Ok(VecVector { coords })
        }
    }
}
impl<T: Clone + Add<Output = T>> Add for VecVector<T> {
    type Output = Result<Self>;
    fn add(self, other: Self) -> Self::Output {
        Self::clone_add(&self, &other)
    }
}
impl<T: Clone + Add<Output = T>> AddAssign<&Self> for VecVector<T> {
    /// # Panics
    /// Panics if `self` and `other` don't have the same size.
    fn add_assign(&mut self, other: &Self) {
        *self = Self::clone_add(self, other).unwrap();
    }
}

impl<T: Clone + Sub<Output = T>> VecVector<T> {
    /// Subtracts two vectors together.
    /// # Errors
    /// Returns [`MismatchedSizes`] if `self and `other` don't have the same length.
    pub fn clone_sub(&self, other: &Self) -> Result<Self> {
        if self.coords.len() != other.coords.len() {
            Err(MismatchedSizes)
        } else {
            let mut coords = Vec::with_capacity(self.coords.len());
            for i in 0..self.coords.len() {
                coords.push(self.coords[i].clone() - other.coords[i].clone());
            }
            Ok(VecVector { coords })
        }
    }
}
impl<T: Clone + Sub<Output = T>> Sub for VecVector<T> {
    type Output = Result<Self>;
    fn sub(self, other: Self) -> Self::Output {
        Self::clone_sub(&self, &other)
    }
}
impl<T: Clone + Sub<Output = T>> SubAssign<&Self> for VecVector<T> {
    /// # Panics
    /// Panics if `self` and `other` don't have the same size.
    fn sub_assign(&mut self, other: &Self) {
        *self = Self::clone_sub(self, other).unwrap();
    }
}

/* ops between vectors and scalars */

impl<T: Clone + Mul<Output = T>> VecVector<T> {
    /// Multiplies a vector by a scalar.
    pub fn clone_mul_scalar(&self, scalar: &T) -> Self {
        let mut coords = Vec::with_capacity(self.coords.len());
        for item in &self.coords {
            coords.push(item.clone() * scalar.clone());
        }
        VecVector { coords }
    }
}
impl<T: Clone + Mul<Output = T>> Mul<T> for VecVector<T> {
    type Output = Self;
    fn mul(self, scalar: T) -> Self::Output {
        Self::clone_mul_scalar(&self, &scalar)
    }
}
impl<T: Clone + Mul<Output = T>> MulAssign<T> for VecVector<T> {
    fn mul_assign(&mut self, scalar: T) {
        *self = Self::clone_mul_scalar(self, &scalar);
    }
}
impl<T: Clone + Mul<Output = T>> Mul<&T> for VecVector<T> {
    type Output = Self;
    fn mul(self, scalar: &T) -> Self::Output {
        Self::clone_mul_scalar(&self, scalar)
    }
}
impl<T: Clone + Mul<Output = T>> MulAssign<&T> for VecVector<T> {
    fn mul_assign(&mut self, scalar: &T) {
        *self = Self::clone_mul_scalar(self, scalar);
    }
}

impl<T: Clone + Div<Output = T>> VecVector<T> {
    /// Divides a vector by a scalar.
    pub fn clone_div_scalar(&self, scalar: &T) -> Self {
        let mut coords = Vec::with_capacity(self.coords.len());
        for item in &self.coords {
            coords.push(item.clone() / scalar.clone());
        }
        VecVector { coords }
    }
}
impl<T: Clone + Div<Output = T>> Div<T> for VecVector<T> {
    type Output = Self;
    fn div(self, scalar: T) -> Self::Output {
        Self::clone_div_scalar(&self, &scalar)
    }
}
impl<T: Clone + Div<Output = T>> DivAssign<T> for VecVector<T> {
    fn div_assign(&mut self, scalar: T) {
        *self = Self::clone_div_scalar(self, &scalar);
    }
}
impl<T: Clone + Div<Output = T>> Div<&T> for VecVector<T> {
    type Output = Self;
    fn div(self, scalar: &T) -> Self::Output {
        Self::clone_div_scalar(&self, scalar)
    }
}
impl<T: Clone + Div<Output = T>> DivAssign<&T> for VecVector<T> {
    fn div_assign(&mut self, scalar: &T) {
        *self = Self::clone_div_scalar(self, scalar);
    }
}
