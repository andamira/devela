// devela_base_core::data::error::other
//
//! Data-related errors.
//
// TOC
// - individual data-related error types:
//   - ElementNotFound
//   - IndexOutOfBounds
//   - InvalidAxisLength
//   - KeyAlreadyExists
//   - MismatchedDimensions
//   - MismatchedIndices
//   - NodeEmpty
//   - NodeLinkNotSet
//   - NodeLinkNotUnique
//   - NotEnoughElements
//   - NotEnoughSpace
//   - PartiallyAdded
// - composite data-related error types:
//   - DataNotEnough:    NotEnoughElements, NotEnoughSpace
//   - MismatchedBounds: MismatchedCapacity, IndexOutOfBounds, MismatchedIndices
//   - PartialSpace:     NotEnoughSpace, PartiallyAdded

use crate::{
    _TAG_DATA, Boundary1d, DOC_MISMATCHED_CAPACITY, Mismatch, MismatchedCapacity, define_error,
};

/* individual errors */

define_error! { individual: pub struct ElementNotFound;
    #[derive(Default)], +location: "data/error", +tag: _TAG_DATA!(),
    DOC_ELEMENT_NOT_FOUND = "The requested element has not been found.",
    self+f => f.write_str(DOC_ELEMENT_NOT_FOUND!()),
}
define_error! { individual:
    /// Optionally contains the given index.
    pub struct IndexOutOfBounds(pub Option<usize>);
    #[derive(Default)], +location: "data/error", +tag: _TAG_DATA!(),
    DOC_INDEX_OUT_OF_BOUNDS = "The given index is out of bounds.",
    self+f => if let Some(i) = self.0 { write!(f, "The given index {i} is out of bounds.")
    } else { f.write_str("The given index is out of bounds.") }
}
define_error! { individual:
    /// Optionally contains the given axis number.
    pub struct InvalidAxisLength(pub Option<usize>);
    #[derive(Default)], +location: "data/error", +tag: _TAG_DATA!(),
    DOC_INVALID_AXIS_LENGTH = "The given axis has an invalid length.",
    self+f => if let Some(n) = self.0 {
        write!(f, "Axis number {n} has 0 length, which is not allowed.")
    } else { f.write_str("One ore more axis have 0 length, which is not allowed.") }
}
define_error! { individual: pub struct KeyAlreadyExists;
    #[derive(Default)], +location: "data/error", +tag: _TAG_DATA!(),
    DOC_KEY_ALREADY_EXISTS = "The key already exists.",
    self+f => f.write_str(DOC_KEY_ALREADY_EXISTS!())
}
define_error! { individual: pub struct MismatchedDimensions(pub Mismatch<usize, usize>);
    #[derive(Default)], +location: "data/error", +tag: _TAG_DATA!(),
    DOC_MISMATCHED_DIMENSIONS = "The dimensions given did not match the elements provided.",
    self+f => write!(f, "Mismatched dimensions: {:?}.", self.0)
}
define_error! { individual: pub struct MismatchedIndices;
    #[derive(Default)], +location: "data/error", +tag: _TAG_DATA!(),
    DOC_MISMATCHED_INDICES = "The given indices does not match the expected order.",
    self+f => f.write_str(DOC_MISMATCHED_INDICES!())
}
define_error! { individual: pub struct NodeEmpty(pub Option<usize>);
    #[derive(Default)], +location: "data/error", +tag: _TAG_DATA!(),
    DOC_NODE_EMPTY = "The node is empty.",
    self+f => if let Some(n) = self.0 { write!(f, "The given node `{n}` is empty.")
    } else { f.write_str(DOC_NODE_EMPTY!()) }
}
define_error! { individual: pub struct NodeLinkNotSet(pub Option<usize>);
    #[derive(Default)], +location: "data/error", +tag: _TAG_DATA!(),
    DOC_NODE_LINK_NOT_SET = "The node link is not set.",
    self+f => if let Some(n) = self.0 { write!(f, "The given node link `{n}` is not set.")
    } else { f.write_str(DOC_NODE_LINK_NOT_SET!()) }
}
define_error! { individual: pub struct NodeLinkNotUnique(pub Option<usize>);
    #[derive(Default)], +location: "data/error", +tag: _TAG_DATA!(),
    DOC_NODE_LINK_NOT_UNIQUE = "The node link is not unique.",
    self+f => if let Some(n) = self.0 { write!(f, "The given node link `{n}` is not unique.")
    } else { f.write_str(DOC_NODE_LINK_NOT_UNIQUE!()) }
}
define_error! { individual:
    /// Optionally contains the minimum number of elements needed.
    pub struct NotEnoughElements(pub Option<usize>);
    #[derive(Default)], +location: "data/error", +tag: _TAG_DATA!(),
    DOC_NOT_ENOUGH_ELEMENTS = "There are not enough elements for the operation.",
    self+f => if let Some(n) = self.0 {
        write!(f, "Not enough elements. Needs at least `{n}` elements.")
    } else { f.write_str("Not enough elements.") }
}
define_error! { individual:
    /// Optionally contains the number of free spaces needed.
    ///
    /// This error represents a contingent failure: the operation may succeed
    /// after a change of state that frees or increases available space.
    pub struct NotEnoughSpace(pub Option<usize>);
    #[derive(Default)], +location: "data/error", +tag: _TAG_DATA!(),
    DOC_NOT_ENOUGH_SPACE = "There is not enough free space for the operation.",
    self+f => if let Some(n) = self.0 {
        write!(f, "Not enough space. Needs at least `{n}` free space for elements.")
    } else { f.write_str("Not enough space.") }
}
define_error! { individual: pub struct PartiallyAdded(pub Option<usize>);
    /// Optionally contains the number of elements added.
    #[derive(Default)], +location: "data/error", +tag: _TAG_DATA!(),
    DOC_PARTIALLY_ADDED = "The operation could only add a subset of the elements.",
    self+f => if let Some(n) = self.0 { write!(f, "Only `{n}` elements could be added.")
    } else { f.write_str("Only a subset of elements could be added.") }
}

/* composite errors */

define_error! { composite: fmt(f)
    +location: "data/error",
    /// An error composite of [`NotEnoughElements`] + [`NotEnoughSpace`].
    ///
    /// Used in methods of:
    /// [`Destaque`],
    /// [`Stack`].
    #[doc = crate::doclink!(custom devela "[`Destaque`]" "data/list/struct.Destaque.html")]
    #[doc = crate::doclink!(custom devela "[`Stack`]" "data/list/struct.Stack.html")]
    pub enum DataNotEnough {
        DOC_NOT_ENOUGH_ELEMENTS: +const Elements(i|0: Option<usize>)  => NotEnoughElements(*i),
        DOC_NOT_ENOUGH_SPACE:    +const Space(i|0: Option<usize>)     => NotEnoughSpace(*i),
    }
}
define_error! { composite: fmt(f)
    +location: "data/error",
    /// An error composite of
    /// [`MismatchedCapacity`] + [`IndexOutOfBounds`] + [`MismatchedIndices`]
    ///
    /// Used in methods of:
    /// [`Array`],
    /// [`Array2d`],
    /// [`bitfield!`][crate::bitfield],
    /// [`BitOps`],
    /// [`Bitwise`][crate::Bitwise].
    #[doc = crate::doclink!(custom devela "[`Array`]" "data/list/struct.Array.html")]
    #[doc = crate::doclink!(custom devela "[`Array2d`]" "data/list/struct.Array2d.html")]
    #[doc = crate::doclink!(custom devela "[`BitOps`]" "data::BitOps")]
    pub enum MismatchedBounds {
        DOC_MISMATCHED_CAPACITY: +const MismatchedCapacity {
            /// Which boundary of the capacity constraint applies.
            bound: Boundary1d,
            /// The value involved in the capacity check, if known.
            value: Option<usize>,
            /// The capacity limit involved in the check, if known.
            limit: Option<usize>
        } => MismatchedCapacity { bound: *bound, value: *value, limit: *limit },

        DOC_INDEX_OUT_OF_BOUNDS: +const
            IndexOutOfBounds(i|0: Option<usize>) => IndexOutOfBounds(*i),
        DOC_MISMATCHED_INDICES: +const
            MismatchedIndices => MismatchedIndices,
    }
}
// MAYBE: Merge with DataNotEnough
define_error! { composite: fmt(f)
    +location: "data/error",
    /// An error composite of [`NotEnoughSpace`] + [`PartiallyAdded`].
    ///
    /// Used in methods of:
    /// [`ArrayUninit`].
    // TEMP
    #[doc = crate::doclink!(custom devela "[`ArrayUninit`]"
        "data/list/array/struct.ArrayUninit.html")]
    pub enum PartialSpace {
        DOC_NOT_ENOUGH_SPACE: +const NotEnoughSpace(i|0: Option<usize>) => NotEnoughSpace(*i),
        DOC_PARTIALLY_ADDED:  +const PartiallyAdded(i|0: Option<usize>) => PartiallyAdded(*i),
    }
}
