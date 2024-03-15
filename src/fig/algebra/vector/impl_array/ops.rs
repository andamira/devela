// devela::fig::algebra::vector::impl_array::ops
//
//! implement overloadable operators
//
// NOTE: Because of T: Clone bound the `_assign` ops take a reference to Rhs.

#![allow(clippy::needless_range_loop)]

use super::super::Vector;
use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

/* ops between vectors */

impl<T: Clone + Add<Output = T>, const D: usize> Vector<T, D> {
    /// Adds two vectors together
    pub fn clone_add(&self, other: &Self) -> Self {
        let mut array: [T; D] = self.array.clone();
        for i in 0..D {
            array[i] = self.array[i].clone() + other.array[i].clone();
        }
        Vector { array }
    }
}
impl<T: Clone + Add<Output = T>, const D: usize> Add for Vector<T, D> {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        Self::clone_add(&self, &other)
    }
}
impl<T: Clone + Add<Output = T>, const D: usize> AddAssign<&Self> for Vector<T, D> {
    fn add_assign(&mut self, other: &Self) {
        *self = Self::clone_add(self, other);
    }
}

impl<T: Clone + Sub<Output = T>, const D: usize> Vector<T, D> {
    /// Subtracts two vectors together.
    pub fn clone_sub(&self, other: &Self) -> Self {
        let mut array: [T; D] = self.array.clone();
        for i in 0..D {
            array[i] = self.array[i].clone() - other.array[i].clone();
        }
        Vector { array }
    }
}
impl<T: Clone + Sub<Output = T>, const D: usize> Sub for Vector<T, D> {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        Self::clone_sub(&self, &other)
    }
}
impl<T: Clone + Sub<Output = T>, const D: usize> SubAssign<&Self> for Vector<T, D> {
    fn sub_assign(&mut self, other: &Self) {
        *self = Self::clone_sub(self, other);
    }
}

/* ops between vectors and scalars */

impl<T: Clone + Mul<Output = T>, const D: usize> Vector<T, D> {
    /// Multiplies a vector by a scalar.
    pub fn clone_mul_scalar(&self, scalar: &T) -> Self {
        let mut array: [T; D] = self.array.clone();
        for i in 0..D {
            array[i] = self.array[i].clone() * scalar.clone();
        }
        Vector { array }
    }
}
impl<T: Clone + Mul<Output = T>, const D: usize> Mul<T> for Vector<T, D> {
    type Output = Self;
    fn mul(self, scalar: T) -> Self::Output {
        Self::clone_mul_scalar(&self, &scalar)
    }
}
impl<T: Clone + Mul<Output = T>, const D: usize> MulAssign<T> for Vector<T, D> {
    fn mul_assign(&mut self, scalar: T) {
        *self = Self::clone_mul_scalar(self, &scalar);
    }
}
impl<T: Clone + Mul<Output = T>, const D: usize> Mul<&T> for Vector<T, D> {
    type Output = Self;
    fn mul(self, scalar: &T) -> Self::Output {
        Self::clone_mul_scalar(&self, scalar)
    }
}
impl<T: Clone + Mul<Output = T>, const D: usize> MulAssign<&T> for Vector<T, D> {
    fn mul_assign(&mut self, scalar: &T) {
        *self = Self::clone_mul_scalar(self, scalar);
    }
}

impl<T: Clone + Div<Output = T>, const D: usize> Vector<T, D> {
    /// Divides a vector by a scalar.
    pub fn clone_div_scalar(&self, scalar: &T) -> Self {
        let mut array: [T; D] = self.array.clone();
        for i in 0..D {
            array[i] = self.array[i].clone() / scalar.clone();
        }
        Vector { array }
    }
}
impl<T: Clone + Div<Output = T>, const D: usize> Div<T> for Vector<T, D> {
    type Output = Self;
    fn div(self, scalar: T) -> Self::Output {
        Self::clone_div_scalar(&self, &scalar)
    }
}
impl<T: Clone + Div<Output = T>, const D: usize> DivAssign<T> for Vector<T, D> {
    fn div_assign(&mut self, scalar: T) {
        *self = Self::clone_div_scalar(self, &scalar);
    }
}
impl<T: Clone + Div<Output = T>, const D: usize> Div<&T> for Vector<T, D> {
    type Output = Self;
    fn div(self, scalar: &T) -> Self::Output {
        Self::clone_div_scalar(&self, scalar)
    }
}
impl<T: Clone + Div<Output = T>, const D: usize> DivAssign<&T> for Vector<T, D> {
    fn div_assign(&mut self, scalar: &T) {
        *self = Self::clone_div_scalar(self, scalar);
    }
}
