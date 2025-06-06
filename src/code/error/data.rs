// devela::code::error::data
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
// - full composite errors:
//   - DataError
//   - DataResult

use crate::{Interval, Mismatch, define_error};

/* individual errors */

define_error! { individual: pub struct DataOverflow(pub Option<usize>);
    DOC_DATA_OVERFLOW = "The value has surpassed the bounds of the representable data space.",
    self+f => if let Some(v) = self.0 {
        write!(f, "The value {v} has surpassed the bounds of the representable data space.")
    } else { write!(f, "The value has surpassed the bounds of the representable data space.") }
}
define_error! { individual: pub struct ElementNotFound;
    DOC_ELEMENT_NOT_FOUND = "The requested element has not been found.",
    self+f => write!(f, "The requested element has not been found."),
}
define_error! { individual: pub struct IndexOutOfBounds(pub Option<usize>);
    DOC_INDEX_OUT_OF_BOUNDS = "The given index is out of bounds.\n\n
Optionally contains the given index.",
    self+f => if let Some(i) = self.0 { write!(f, "The given index {i} is out of bounds.")
    } else { write!(f, "The given index is out of bounds.") }
}
define_error! { individual: pub struct InvalidAxisLength(pub Option<usize>);
    DOC_INVALID_AXIS_LENGTH = "The given axis has an invalid length.\n\n
Optionally contains the given axis number.",
    self+f => if let Some(n) = self.0 {
        write!(f, "Axis number {n} has 0 length, which is not allowed.")
    } else { write!(f, "One ore more axis have 0 length, which is not allowed.") }
}
define_error! { individual: pub struct KeyAlreadyExists;
    DOC_KEY_ALREADY_EXISTS = "The key already exists.",
    self+f => write!(f, "The key already exists.")
}
define_error! { individual: pub struct MismatchedCapacity(pub Mismatch<Interval<usize>, usize>);
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
    DOC_MISMATCHED_DIMENSIONS = "The dimensions given did not match the elements provided.",
    self+f => write!(f, "Mismatched dimensions: {:?}.", self.0)
}
define_error! { individual: pub struct MismatchedIndices;
    DOC_MISMATCHED_INDICES = "The given indices does not match the expected order.",
    self+f => write!(f, "The given indices does not match the expected order.")
}
define_error! { individual: pub struct NodeEmpty(pub Option<usize>);
    DOC_NODE_EMPTY = "The node is empty.",
    self+f => if let Some(n) = self.0 { write!(f, "The given node `{n}` is empty.")
    } else { write!(f, "The node is empty.") }
}
define_error! { individual: pub struct NodeLinkNotSet(pub Option<usize>);
    DOC_NODE_LINK_NOT_SET = "The link is not set.",
    self+f => if let Some(n) = self.0 { write!(f, "The given node link `{n}` is not set.")
    } else { write!(f, "The node link is not set.") }
}
define_error! { individual: pub struct NodeLinkNotUnique(pub Option<usize>);
    DOC_NODE_LINK_NOT_UNIQUE = "The link is not unique.",
    self+f => if let Some(n) = self.0 { write!(f, "The given node link `{n}` is not unique.")
    } else { write!(f, "The node link is not unique.") }
}
define_error! { individual: pub struct NotEnoughElements(pub Option<usize>);
    DOC_NOT_ENOUGH_ELEMENTS = "There are not enough elements for the operation.\n\n
Optionally contains the minimum number of elements needed.",
    self+f => if let Some(n) = self.0 {
        write!(f, "Not enough elements. Needs at least `{n}` elements.")
    } else { write!(f, "Not enough elements.") }
}
define_error! { individual: pub struct NotEnoughSpace(pub Option<usize>);
    DOC_NOT_ENOUGH_SPACE = "There is not enough free space for the operation.\n\n
Optionally contains the number of free spaces needed.",
    self+f => if let Some(n) = self.0 {
        write!(f, "Not enough space. Needs at least `{n}` free space for elements.")
    } else { write!(f, "Not enough space.") }
}
define_error! { individual: pub struct PartiallyAdded(pub Option<usize>);
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
        DOC_NOT_ENOUGH_ELEMENTS:    Elements(i|0: Option<usize>)  => NotEnoughElements(*i),
        DOC_NOT_ENOUGH_SPACE:       Space(i|0: Option<usize>)     => NotEnoughSpace(*i),
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
    /// [`BitOps`][crate::BitOps],
    /// [`Bitwise`][crate::Bitwise].
    pub enum MismatchedBounds {
        DOC_DATA_OVERFLOW:
            DataOverflow(i|0: Option<usize>) => DataOverflow(*i),
        DOC_INDEX_OUT_OF_BOUNDS:
            IndexOutOfBounds(i|0: Option<usize>) => IndexOutOfBounds(*i),
        DOC_MISMATCHED_CAPACITY:
            MismatchedCapacity(c|0: Mismatch<Interval<usize>, usize>) => MismatchedCapacity(*c),
        DOC_MISMATCHED_INDICES:
            MismatchedIndices => MismatchedIndices,
    }
}
// MAYBE: Merge with DataNotEnough
define_error! { composite: fmt(f)
    /// An error composite of [`NotEnoughSpace`] + [`PartiallyAdded`].
    ///
    /// Used in methods of:
    /// [`ArrayUninit`][crate::ArrayUninit].
    pub enum PartialSpace {
        DOC_NOT_ENOUGH_SPACE:   NotEnoughSpace(i|0: Option<usize>) => NotEnoughSpace(*i),
        DOC_PARTIALLY_ADDED:    PartiallyAdded(i|0: Option<usize>) => PartiallyAdded(*i),
    }
}

#[cfg(all(feature = "error", data··))]
pub use full_composite::*;
#[cfg(all(feature = "error", data··))]
#[cfg_attr(nightly_doc, doc(cfg(all(feature = "error", data··))))]
mod full_composite {
    use super::*;
    use crate::{
        DOC_NOT_IMPLEMENTED, DOC_NOT_SUPPORTED, NotAvailable, NotImplemented, NotSupported,
    };

    define_error! { composite: fmt(f)
        /// A data-related composite error.
        #[non_exhaustive]
        pub enum DataError {
            DOC_NOT_IMPLEMENTED: NotImplemented => NotImplemented,
            DOC_NOT_SUPPORTED: NotSupported => NotSupported,
            //
            DOC_DATA_OVERFLOW:
                DataOverflow(i|0: Option<usize>) => DataOverflow(*i),
            DOC_ELEMENT_NOT_FOUND:
                ElementNotFound => ElementNotFound,
            DOC_INDEX_OUT_OF_BOUNDS:
                IndexOutOfBounds(i|0: Option<usize>) => IndexOutOfBounds(*i),
            DOC_INVALID_AXIS_LENGTH:
                InvalidAxisLength(i|0: Option<usize>) => InvalidAxisLength(*i),
            DOC_KEY_ALREADY_EXISTS:
                KeyAlreadyExists => KeyAlreadyExists,
            DOC_MISMATCHED_CAPACITY:
                MismatchedCapacity(c|0: Mismatch<Interval<usize>, usize>) => MismatchedCapacity(*c),
            DOC_MISMATCHED_DIMENSIONS:
                MismatchedDimensions(d|0: Mismatch<usize, usize>) => MismatchedDimensions(*d),
            DOC_MISMATCHED_INDICES:
                MismatchedIndices => MismatchedIndices,
            DOC_NODE_EMPTY:
                NodeEmpty(i|0: Option<usize>) => NodeEmpty(*i),
            DOC_NODE_LINK_NOT_SET:
                NodeLinkNotSet(i|0: Option<usize>) => NodeLinkNotSet(*i),
            DOC_NODE_LINK_NOT_UNIQUE:
                NodeLinkNotUnique(i|0: Option<usize>) => NodeLinkNotUnique(*i),
            DOC_NOT_ENOUGH_ELEMENTS:
                NotEnoughElements(i|0: Option<usize>) => NotEnoughElements(*i),
            DOC_NOT_ENOUGH_SPACE:
                NotEnoughSpace(i|0: Option<usize>) => NotEnoughSpace(*i),
            DOC_PARTIALLY_ADDED:
                PartiallyAdded(i|0: Option<usize>) => PartiallyAdded(*i),
        }
    }
    define_error! { composite: from(f): NotAvailable, for: DataError {
        NotImplemented => NotImplemented,
        NotSupported => NotSupported,
    }}
    define_error! { composite: from(f): DataNotEnough, for: DataError {
        Elements(i) => NotEnoughElements(i),
        Space(i) => NotEnoughSpace(i),
    }}
    define_error! { composite: from(f): PartialSpace, for: DataError {
        NotEnoughSpace(i) => NotEnoughSpace(i),
        PartiallyAdded(i) => PartiallyAdded(i),
    }}
    define_error! { composite: from(f): MismatchedBounds, for: DataError {
        DataOverflow(i) => DataOverflow(i),
        IndexOutOfBounds(i) => IndexOutOfBounds(i),
        MismatchedCapacity(i) => MismatchedCapacity(i),
        MismatchedIndices => MismatchedIndices,
    }}

    #[doc = crate::TAG_RESULT!()]
    /// A data-related result.
    pub type DataResult<T> = crate::Result<T, DataError>;
}
