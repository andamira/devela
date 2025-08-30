// devela_base::data::errors
//
//! Data-related errors.
//
// TOC
// - individual data-related error types:
//   - DataOverflow
//   - ElementNotFound
//   - IndexOutOfBounds
//   - InvalidAxisLength
//   - KeyAlreadyExists
//   - MismatchedCapacity
//   - MismatchedDimensions
//   - MismatchedIndices
//   - NodeEmpty
//   - NodeLinkNotSet
//   - NodeLinkNotUnique
//   - NotEnoughElements
//   - NotEnoughSpace
//   - PartiallyAdded
// - partial composite errors:
//   - DataNotEnough:    NotEnoughElements, NotEnoughSpace
//   - MismatchedBounds: DataOverflow, IndexOutOfBounds, MismatchedIndices, MismatchedCapacity
//   - PartialSpace:     NotEnoughSpace, PartiallyAdded

use crate::{Interval, Mismatch, TAG_DATA, define_error};

/* individual errors */

define_error! { individual: pub struct DataOverflow(pub Option<usize>);
    +tag: TAG_DATA!(),
    DOC_DATA_OVERFLOW = "The value has surpassed the bounds of the representable data space.",
    self+f => if let Some(v) = self.0 {
        write!(f, "The value {v} has surpassed the bounds of the representable data space.")
    } else { write!(f, "The value has surpassed the bounds of the representable data space.") }
}
define_error! { individual: pub struct ElementNotFound;
    +tag: TAG_DATA!(),
    DOC_ELEMENT_NOT_FOUND = "The requested element has not been found.",
    self+f => write!(f, "The requested element has not been found."),
}
define_error! { individual: pub struct IndexOutOfBounds(pub Option<usize>);
    +tag: TAG_DATA!(),
    DOC_INDEX_OUT_OF_BOUNDS = "The given index is out of bounds.\n\n
Optionally contains the given index.",
    self+f => if let Some(i) = self.0 { write!(f, "The given index {i} is out of bounds.")
    } else { write!(f, "The given index is out of bounds.") }
}
define_error! { individual: pub struct InvalidAxisLength(pub Option<usize>);
    +tag: TAG_DATA!(),
    DOC_INVALID_AXIS_LENGTH = "The given axis has an invalid length.\n\n
Optionally contains the given axis number.",
    self+f => if let Some(n) = self.0 {
        write!(f, "Axis number {n} has 0 length, which is not allowed.")
    } else { write!(f, "One ore more axis have 0 length, which is not allowed.") }
}
define_error! { individual: pub struct KeyAlreadyExists;
    +tag: TAG_DATA!(),
    DOC_KEY_ALREADY_EXISTS = "The key already exists.",
    self+f => write!(f, "The key already exists.")
}
define_error! { individual: pub struct MismatchedCapacity(pub Mismatch<Interval<usize>, usize>);
    +tag: TAG_DATA!(),
    DOC_MISMATCHED_CAPACITY = "The given capacity did not match the required constraints.",
    self+f => write!(f, "Mismatched capacity: {:?}.", self.0)
}
impl MismatchedCapacity {
    /// Creates a mismatch where `need` is an [`Interval::closed`], and `have` is outside it.
    #[must_use]
    pub const fn closed(lower: usize, upper: usize, have: usize) -> Self {
        Self(Mismatch::in_closed_interval(lower, upper, have, DOC_MISMATCHED_CAPACITY!()))
    }
    /// Creates a mismatch where `need` is an [`Interval::closed_open`], and `have` is outside it.
    #[must_use]
    pub const fn closed_open(lower: usize, upper: usize, have: usize) -> Self {
        Self(Mismatch::in_closed_open_interval(lower, upper, have, DOC_MISMATCHED_CAPACITY!()))
    }
}
define_error! { individual: pub struct MismatchedDimensions(pub Mismatch<usize, usize>);
    +tag: TAG_DATA!(),
    DOC_MISMATCHED_DIMENSIONS = "The dimensions given did not match the elements provided.",
    self+f => write!(f, "Mismatched dimensions: {:?}.", self.0)
}
define_error! { individual: pub struct MismatchedIndices;
    +tag: TAG_DATA!(),
    DOC_MISMATCHED_INDICES = "The given indices does not match the expected order.",
    self+f => write!(f, "The given indices does not match the expected order.")
}
define_error! { individual: pub struct NodeEmpty(pub Option<usize>);
    +tag: TAG_DATA!(),
    DOC_NODE_EMPTY = "The node is empty.",
    self+f => if let Some(n) = self.0 { write!(f, "The given node `{n}` is empty.")
    } else { write!(f, "The node is empty.") }
}
define_error! { individual: pub struct NodeLinkNotSet(pub Option<usize>);
    +tag: TAG_DATA!(),
    DOC_NODE_LINK_NOT_SET = "The link is not set.",
    self+f => if let Some(n) = self.0 { write!(f, "The given node link `{n}` is not set.")
    } else { write!(f, "The node link is not set.") }
}
define_error! { individual: pub struct NodeLinkNotUnique(pub Option<usize>);
    +tag: TAG_DATA!(),
    DOC_NODE_LINK_NOT_UNIQUE = "The link is not unique.",
    self+f => if let Some(n) = self.0 { write!(f, "The given node link `{n}` is not unique.")
    } else { write!(f, "The node link is not unique.") }
}
define_error! { individual: pub struct NotEnoughElements(pub Option<usize>);
    +tag: TAG_DATA!(),
    DOC_NOT_ENOUGH_ELEMENTS = "There are not enough elements for the operation.\n\n
Optionally contains the minimum number of elements needed.",
    self+f => if let Some(n) = self.0 {
        write!(f, "Not enough elements. Needs at least `{n}` elements.")
    } else { write!(f, "Not enough elements.") }
}
define_error! { individual: pub struct NotEnoughSpace(pub Option<usize>);
    +tag: TAG_DATA!(),
    DOC_NOT_ENOUGH_SPACE = "There is not enough free space for the operation.\n\n
Optionally contains the number of free spaces needed.",
    self+f => if let Some(n) = self.0 {
        write!(f, "Not enough space. Needs at least `{n}` free space for elements.")
    } else { write!(f, "Not enough space.") }
}
define_error! { individual: pub struct PartiallyAdded(pub Option<usize>);
    +tag: TAG_DATA!(),
    DOC_PARTIALLY_ADDED = "The operation could only add a subset of the elements.\n\n
Optionally contains the number of elements added.",
    self+f => if let Some(n) = self.0 { write!(f, "Only `{n}` elements could be added.")
    } else { write!(f, "Only a subset of elements could be added.") }
}

/* composite errors */

define_error! { composite: fmt(f)
    /// An error composite of [`NotEnoughElements`] + [`NotEnoughSpace`].
    ///
    /// Used in methods of:
    /// [`Destaque`][crate::Destaque],
    /// [`Stack`][crate::Stack].
    pub enum DataNotEnough {
        DOC_NOT_ENOUGH_ELEMENTS: +const Elements(i|0: Option<usize>)  => NotEnoughElements(*i),
        DOC_NOT_ENOUGH_SPACE:    +const Space(i|0: Option<usize>)     => NotEnoughSpace(*i),
    }
}
define_error! { composite: fmt(f)
    /// An error composite of
    /// [`DataOverflow`] + [`IndexOutOfBounds`] +
    /// [`MismatchedIndices`] + [`MismatchedCapacity`].
    ///
    /// Used in methods of:
    /// [`Array`][crate::Array],
    /// [`Array2d`][crate::Array2d],
    /// [`bitfield!`][crate::bitfield],
    /// [`BitOps`],
    /// [`Bitwise`][crate::Bitwise].
    #[doc = crate::_doclink!("[`BitOps`]" devela "data::BitOps")]
    pub enum MismatchedBounds {
        DOC_DATA_OVERFLOW: +const
            DataOverflow(i|0: Option<usize>) => DataOverflow(*i),
        DOC_INDEX_OUT_OF_BOUNDS: +const
            IndexOutOfBounds(i|0: Option<usize>) => IndexOutOfBounds(*i),
        DOC_MISMATCHED_CAPACITY: +const
            MismatchedCapacity(c|0: Mismatch<Interval<usize>, usize>) => MismatchedCapacity(*c),
        DOC_MISMATCHED_INDICES: +const
            MismatchedIndices => MismatchedIndices,
    }
}
// MAYBE: Merge with DataNotEnough
define_error! { composite: fmt(f)
    /// An error composite of [`NotEnoughSpace`] + [`PartiallyAdded`].
    ///
    /// Used in methods of:
    /// [`ArrayUninit`].
    // TEMP
    #[doc = crate::_doclink!("[`ArrayUninit`]" devela "data/list/array/struct.ArrayUninit.html")]
    pub enum PartialSpace {
        DOC_NOT_ENOUGH_SPACE: +const NotEnoughSpace(i|0: Option<usize>) => NotEnoughSpace(*i),
        DOC_PARTIALLY_ADDED:  +const PartiallyAdded(i|0: Option<usize>) => PartiallyAdded(*i),
    }
}
