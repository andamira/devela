// devela::num::geom::shape::angle::impl::core_traits
//
//!
//

use crate::{Angle, ConstDefault, Debug, FmtResult, Formatter, Ordering};

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
impl<T: Debug> Debug for Angle<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult<()> {
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
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

// T:Ord
impl<T: Ord> Ord for Angle<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}
