// devela::num::geom::metric::angle::impl::core_traits
//
//!
//

use crate::{Angle, ConstInit, Debug, FmtResult, Formatter, Ordering};

/* Clone, Copy */

// T:Clone
impl<T: Clone> Clone for Angle<T> {
    fn clone(&self) -> Self {
        Self::new(self.turn.clone())
    }
}

// T:Copy
impl<T: Copy> Copy for Angle<T> {}

/* Default, ConstInit */

impl<T: Default> Default for Angle<T> {
    /// Returns a `Angle`, allocated in the stack,
    /// using the default value to fill the data.
    fn default() -> Self {
        Self::new(T::default())
    }
}

// S:Bare + T:ConstInit
impl<T: ConstInit> ConstInit for Angle<T> {
    /// Returns a Angle, allocated in the stack,
    /// using the default value to fill the data.
    const INIT: Self = { Self::new(T::INIT) };
}

// T:Debug
impl<T: Debug> Debug for Angle<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult<()> {
        write!(f, "Angle({:?})", self.turn)
    }
}

// T:PartialEq
impl<T: PartialEq> PartialEq for Angle<T> {
    fn eq(&self, other: &Self) -> bool {
        self.turn == other.turn
    }
}
// T:Eq
impl<T: Eq> Eq for Angle<T> {}

// T:PartialOrd
impl<T: PartialOrd> PartialOrd for Angle<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.turn.partial_cmp(&other.turn)
    }
}

// T:Ord
impl<T: Ord> Ord for Angle<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.turn.cmp(&other.turn)
    }
}
