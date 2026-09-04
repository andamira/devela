// devela/src/data/topol/ord/concat.rs
//
//! Defines [`Concat`].
//

use crate::ConstInit;

#[doc = crate::_tags!(data topol)]
/// An ordered binary composition of two parts.
#[doc = crate::_doc_meta! {
    location("data/topol/ord", struct Concat),
}]
/// The `left` part precedes the `right` part in the composition.
///
/// `Concat` describes only the ordered composition itself. It does not
/// imply contiguity, homogeneous part types, storage, ownership, or
/// geometric extent.
#[must_use]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Concat<L, R = L> {
    /// The left, preceding part.
    pub left: L,
    /// The right, following part.
    pub right: R,
}

impl<L, R> Concat<L, R> {
    /// Creates an ordered composition of `left` followed by `right`.
    pub const fn new(left: L, right: R) -> Self {
        Self { left, right }
    }

    /// Decomposes this composition into its ordered parts.
    pub fn into_parts(self) -> (L, R) {
        (self.left, self.right)
    }
    /// Borrows its ordered parts.
    pub const fn as_parts(&self) -> (&L, &R) {
        (&self.left, &self.right)
    }
    /// Borrows both parts while preserving the composition structure.
    pub const fn as_ref(&self) -> Concat<&L, &R> {
        Concat::new(&self.left, &self.right)
    }
    /// Exclusively borrows both parts while preserving the composition structure.
    pub const fn as_mut(&mut self) -> Concat<&mut L, &mut R> {
        Concat::new(&mut self.left, &mut self.right)
    }

    /// Exchanges the two immediate parts.
    pub fn swapped(self) -> Concat<R, L> {
        Concat::new(self.right, self.left)
    }
}

impl<L: Copy, R: Copy> Concat<L, R> {
    /// Returns copies of its ordered parts.
    pub const fn copy_parts(&self) -> (L, R) {
        (self.left, self.right)
    }
    /// Returns a copy with the two immediate parts exchanged.
    pub const fn swapped_copy(&self) -> Concat<R, L> {
        Concat::new(self.right, self.left)
    }
}

impl<L: ConstInit, R: ConstInit> ConstInit for Concat<L, R> {
    const INIT: Self = Self::new(L::INIT, R::INIT);
}
