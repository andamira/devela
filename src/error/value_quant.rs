// devela::error::value_quant
//
//!
//

/// A value with associated quantification.
#[must_use]
pub struct ValueQuant<V, Q> {
    /// The main value.
    pub v: V,
    /// The quantification of the value.
    pub q: Q,
}

impl<V, Q> ValueQuant<V, Q> {
    /// A constructor with the given `value` and `quant`.
    #[inline]
    pub const fn new(value: V, quant: Q) -> ValueQuant<V, Q> {
        ValueQuant { v: value, q: quant }
    }

    /// Constructs itself from a tuple.
    #[inline] #[rustfmt::skip]
    pub fn from_vq(tuple: (V, Q)) -> ValueQuant<V, Q> {
        ValueQuant { v: tuple.0, q: tuple.1, }
    }

    /// Transforms itself into a tuple.
    #[inline] #[must_use] #[rustfmt::skip]
    pub fn vq(self) -> (V, Q) { (self.v, self.q) }

    /// Returns a tuple of shared references to its fields.
    #[inline] #[must_use] #[rustfmt::skip]
    pub fn vq_ref(&self) -> (&V, &Q) { (&self.v, &self.q) }

    /// Returns a tuple of exclusive references to its fields.
    #[inline] #[must_use] #[rustfmt::skip]
    pub fn vq_mut(&mut self) -> (&mut V, &mut Q) { (&mut self.v, &mut self.q) }
}

impl<V: Copy, Q: Copy> ValueQuant<V, Q> {
    /// Constructs itself from a tuple, in compile-time.
    #[inline] #[rustfmt::skip]
    pub const fn from_vq_const(tuple: (V, Q)) -> ValueQuant<V, Q> {
        ValueQuant { v: tuple.0, q: tuple.1,
        }
    }

    /// Transforms itself into a tuple, in compile-time.
    #[inline] #[must_use] #[rustfmt::skip]
    pub const fn vq_const(self) -> (V, Q) { (self.v, self.q) }
}

mod core_impls {
    use super::ValueQuant;
    use core::{
        cmp::Ordering,
        fmt,
        hash::{Hash, Hasher},
    };

    impl<V: Clone, Q: Clone> Clone for ValueQuant<V, Q> {
        #[inline]
        fn clone(&self) -> Self {
            Self { v: self.v.clone(), q: self.q.clone() }
        }
    }
    impl<V: Copy, Q: Copy> Copy for ValueQuant<V, Q> {}

    impl<V: Default, Q: Default> Default for ValueQuant<V, Q> {
        /// Returns an empty ValueQuant with `None` for both fields.
        #[inline]
        fn default() -> Self {
            Self { v: Default::default(), q: Default::default() }
        }
    }

    impl<V: fmt::Debug, Q: fmt::Debug> fmt::Debug for ValueQuant<V, Q> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let mut debug = f.debug_struct("ValueQuant");
            debug.field("v", &self.v);
            debug.field("q", &self.q);
            debug.finish()
        }
    }

    impl<V: fmt::Display, Q: fmt::Display> fmt::Display for ValueQuant<V, Q> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "Value: {}, Quant: {}", self.v, self.q)
        }
    }

    impl<V: PartialEq, Q: PartialEq> PartialEq for ValueQuant<V, Q> {
        #[inline]
        fn eq(&self, other: &Self) -> bool {
            self.v == other.v && self.q == other.q
        }
    }
    impl<V: Eq, Q: Eq> Eq for ValueQuant<V, Q> {}
    // with a tuple:
    impl<V: PartialEq, Q: PartialEq> PartialEq<(V, Q)> for ValueQuant<V, Q> {
        #[inline]
        fn eq(&self, other: &(V, Q)) -> bool {
            self.v == other.0 && self.q == other.1
        }
    }

    impl<V: PartialOrd, Q: PartialOrd> PartialOrd for ValueQuant<V, Q> {
        /// Compare `value` first. If they are equal, then compare `quant`.
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            match self.v.partial_cmp(&other.v) {
                Some(Ordering::Equal) => self.q.partial_cmp(&other.q),
                other => other,
            }
        }
    }
    impl<V: Ord, Q: Ord> Ord for ValueQuant<V, Q> {
        /// Compare `value` first. If they are equal, then compare `quant`.
        fn cmp(&self, other: &Self) -> Ordering {
            match self.v.cmp(&other.v) {
                Ordering::Equal => self.q.cmp(&other.q),
                order => order,
            }
        }
    }

    impl<V: Hash, Q: Hash> Hash for ValueQuant<V, Q> {
        #[inline]
        fn hash<HR: Hasher>(&self, state: &mut HR) {
            self.v.hash(state);
            self.q.hash(state);
        }
    }
}
