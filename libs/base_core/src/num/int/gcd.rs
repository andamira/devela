// devela_base_core::num::int::gcd
//
//! (Extended) Greatest Common Divisor return type.
//

use ::core::fmt;

#[doc = crate::TAG_NUM!()]
#[doc = crate::TAG_RESULT!()]
/// A return type for the calculated
/// <abbr title="Greatest Common Divisor">GCD</abbr> and the Bézout coeficients.
///
/// The coefficients are the solutions to the equation $ \text{gcd}(a, b) = a*x + b*y $.
#[must_use]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct GcdReturn<G, C> {
    /// The greatest common divisor.
    pub gcd: G,
    /// The first Bézout's coefficient `x`.
    pub x: C,
    /// The second Bézout's coefficient `y`.
    pub y: C,
}

impl<G, C> GcdReturn<G, C> {
    /// Constructs a new `GcdReturn`.
    pub const fn new(gcd: G, x: C, y: C) -> Self {
        GcdReturn { gcd, x, y }
    }

    /// Returns the values as a tuple.
    #[must_use]
    pub fn as_tuple(self) -> (G, C, C) {
        (self.gcd, self.x, self.y)
    }
}
impl<G: Copy, C: Copy> GcdReturn<G, C> {
    /// Returns the values as a tuple, in compile-time.
    #[must_use]
    pub const fn as_tuple_copy(self) -> (G, C, C) {
        (self.gcd, self.x, self.y)
    }
}

impl<T> GcdReturn<T, T> {
    /// Returns the values as an array, if all are of the same type.
    #[must_use]
    pub fn as_array(self) -> [T; 3] {
        [self.gcd, self.x, self.y]
    }
}
impl<T: Copy> GcdReturn<T, T> {
    /// Returns the values as an array, if all are of the same type.
    #[must_use]
    pub const fn as_array_copy(self) -> [T; 3] {
        [self.gcd, self.x, self.y]
    }
}

impl<G: fmt::Display, C: fmt::Display> fmt::Display for GcdReturn<G, C> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "gcd: {}, x: {}, y: {}", self.gcd, self.x, self.y)
    }
}
