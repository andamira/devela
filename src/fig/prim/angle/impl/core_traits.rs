// devela::fig::prim::angle::core_traits
//
//!
//

use crate::{code::ConstDefault, fig::Angle};
use core::{cmp::Ordering, fmt};

/* Clone, Copy */

// T:Clone
impl<T: Clone> Clone for Angle<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

// T:Copy
impl<T: Copy> Copy for Angle<T> {}

/* Default, ConstDefault */

impl<T: Default> Default for Angle<T> {
    /// Returns a `Angle`, allocated in the stack,
    /// using the default value to fill the data.
    fn default() -> Self {
        Self(T::default())
    }
}

// S:Bare + T:ConstDefault
impl<T: ConstDefault> ConstDefault for Angle<T> {
    /// Returns a Angle, allocated in the stack,
    /// using the default value to fill the data.
    const DEFAULT: Self = { Self(T::DEFAULT) };
}

// T:Debug
impl<T: fmt::Debug> fmt::Debug for Angle<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Angle({:?})", self.0)
    }
}

// T:PartialEq
impl<T: PartialEq> PartialEq for Angle<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
// T:Eq
impl<T: Eq> Eq for Angle<T> {}

// T:PartialOrd
impl<T: PartialOrd> PartialOrd for Angle<T> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

// T:Ord
impl<T: Ord> Ord for Angle<T> {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}
