// devela/src/data/topol/ord/seq.rs
//
//! Defines [`SeqNode`].
//

use crate::{Concat, ConstInit};

#[doc = crate::_tags!(data topol)]
/// A node in an ordered sequence topology, either a leaf or a binary concatenation.
#[doc = crate::_doc_meta! {
    location("data/topol/ord", enum SeqNode),
}]
/// A `Leaf` terminates the topology,
/// while `Concat` refers to two child nodes in composition order.
///
/// Node storage and child resolution are external to this type.
#[must_use]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SeqNode<L, N> {
    /// A terminal leaf.
    Leaf(L),

    /// An ordered concatenation of two child nodes.
    Concat(Concat<N>),
}

impl<L, N> SeqNode<L, N> {
    /// Creates a terminal leaf.
    pub const fn leaf(leaf: L) -> Self {
        Self::Leaf(leaf)
    }
    /// Creates an ordered concatenation of two child nodes.
    pub const fn concat(left: N, right: N) -> Self {
        Self::Concat(Concat::new(left, right))
    }
    /// Creates a node from an existing concatenation.
    pub const fn from_concat(concat: Concat<N>) -> Self {
        Self::Concat(concat)
    }

    /// Returns whether this is a leaf.
    pub const fn is_leaf(&self) -> bool {
        matches!(self, Self::Leaf(_))
    }
    /// Returns whether this is a concatenation.
    pub const fn is_concat(&self) -> bool {
        matches!(self, Self::Concat(_))
    }

    /// Borrows the leaf, if present.
    pub const fn as_leaf(&self) -> Option<&L> {
        match self {
            Self::Leaf(leaf) => Some(leaf),
            Self::Concat(_) => None,
        }
    }
    /// Borrows the concatenation, if present.
    pub const fn as_concat(&self) -> Option<&Concat<N>> {
        match self {
            Self::Leaf(_) => None,
            Self::Concat(concat) => Some(concat),
        }
    }
    /// Borrows the contents while preserving the node structure.
    pub const fn as_ref(&self) -> SeqNode<&L, &N> {
        match self {
            Self::Leaf(leaf) => SeqNode::Leaf(leaf),
            Self::Concat(concat) => SeqNode::Concat(concat.as_ref()),
        }
    }
    /// Mutably borrows the contents while preserving the node structure.
    pub const fn as_mut(&mut self) -> SeqNode<&mut L, &mut N> {
        match self {
            Self::Leaf(leaf) => SeqNode::Leaf(leaf),
            Self::Concat(concat) => SeqNode::Concat(concat.as_mut()),
        }
    }
}

impl<L: ConstInit, N> ConstInit for SeqNode<L, N> {
    const INIT: Self = Self::Leaf(L::INIT);
}
