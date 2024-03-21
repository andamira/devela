// devela::num::gcd
//
//! Extended GCD return type.
//

use core::fmt;

/// A return type for the calculated
/// <abbr title="Greatest Common Divisor">GCD</abbr> and the Bézout coeficients.
///
/// The coefficients are solutions to the equation $ gcd(a, b) = a*x + b*y $.
#[must_use]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GcdExt<G, C> {
    /// The greatest common divisor.
    pub gcd: G,
    /// The first Bézout's coefficient `x`.
    pub x: C,
    /// The second Bézout's coefficient `y`.
    pub y: C,
}

impl<G, C> GcdExt<G, C> {
    /// Constructs a new `GcdExt`.
    #[inline]
    pub const fn new(gcd: G, x: C, y: C) -> Self {
        GcdExt { gcd, x, y }
    }

    /// Returns the values as a tuple.
    #[inline]
    #[must_use]
    pub fn as_tuple(self) -> (G, C, C) {
        (self.gcd, self.x, self.y)
    }
}
impl<G: Copy, C: Copy> GcdExt<G, C> {
    /// Returns the values as a tuple, in compile-time.
    #[inline]
    #[must_use]
    pub const fn as_tuple_const(self) -> (G, C, C) {
        (self.gcd, self.x, self.y)
    }
}

impl<T> GcdExt<T, T> {
    /// Returns the values as an array, if all are of the same type.
    #[inline]
    #[must_use]
    pub fn as_array(self) -> [T; 3] {
        [self.gcd, self.x, self.y]
    }
}
impl<T: Copy> GcdExt<T, T> {
    /// Returns the values as an array, if all are of the same type.
    #[inline]
    #[must_use]
    pub const fn as_array_const(self) -> [T; 3] {
        [self.gcd, self.x, self.y]
    }
}

impl<G: fmt::Display, C: fmt::Display> fmt::Display for GcdExt<G, C> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "gcd: {}, x: {}, y: {}", self.gcd, self.x, self.y)
    }
}
