// devela::num::geom::shape::point::core_traits
//
//!
//

use crate::{
    ConstDefault, Debug, Display, ExtArray, FmtResult, Formatter, Hash, Hasher, Ordering, Point,
    array_init,
};

impl<T: Clone, const D: usize> Clone for Point<T, D> {
    fn clone(&self) -> Self {
        Self::new(self.coords.clone())
    }
}
impl<T: Copy, const D: usize> Copy for Point<T, D> {}

impl<T: Default, const D: usize> Default for Point<T, D> {
    fn default() -> Self {
        Self::new(array_init![default [T; D], "safe_num", "unsafe_array"])
    }
}
impl<T: ConstDefault, const D: usize> ConstDefault for Point<T, D> {
    const DEFAULT: Self = Self::new(array_init![const_default [T; D]]);
}

impl<T: Debug, const D: usize> Debug for Point<T, D> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult<()> {
        f.debug_tuple("Point").field(&self.coords).finish()
    }
}
impl<T: Display, const D: usize> Display for Point<T, D> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult<()> {
        write!(f, "Point {{ coords: {} }}", self.coords.fmt())
    }
}

impl<T: PartialEq, const D: usize> PartialEq for Point<T, D> {
    fn eq(&self, other: &Self) -> bool {
        self.coords == other.coords
    }
}
impl<T: Eq, const D: usize> Eq for Point<T, D> {}

impl<T: PartialOrd, const D: usize> PartialOrd for Point<T, D> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.coords.partial_cmp(&other.coords)
    }
}
impl<T: Ord, const D: usize> Ord for Point<T, D> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.coords.cmp(&other.coords)
    }
}

impl<T: Hash, const D: usize> Hash for Point<T, D> {
    fn hash<HR: Hasher>(&self, state: &mut HR) {
        self.coords.hash(state);
    }
}
