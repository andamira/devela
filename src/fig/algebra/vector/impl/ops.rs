// devela::fig::algebra::vector::impl::ops
//
//! implement overloadable operators
//
// NOTE: Because of T: Clone, `_assign` ops have to take a reference to other.

#![allow(clippy::needless_range_loop)]

use super::super::Vector;
use core::ops::{Add, AddAssign};

impl<T: Clone + Add<Output = T>, const D: usize> Vector<T, D> {
    /// Adds two vectors together, returns a result to handle potential overflow.
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
