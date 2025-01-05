// devela::data::error
//
//! Data-related error types.
//
// TOC
// - standalone data-related error types:
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

use crate::{impl_error, Mismatch};

impl_error! { individual: pub struct DataOverflow(pub Option<usize>);
    DOC_DATA_OVERFLOW = "The value has surpassed the bounds of the representable data space.",
    self+f => if let Some(v) = self.0 {
        write!(f, "The value {v} has surpassed the bounds of the representable data space.")
    } else { write!(f, "The value has surpassed the bounds of the representable data space.") }
}
impl_error! { individual: pub struct ElementNotFound;
    DOC_ELEMENT_NOT_FOUND = "The requested element has not been found.",
    self+f => write!(f, "The requested element has not been found."),
}
impl_error! { individual: pub struct IndexOutOfBounds(pub Option<usize>);
    DOC_INDEX_OUT_OF_BOUNDS = "The given index is out of bounds.\n\n
Optionally contains the given index.",
    self+f => if let Some(i) = self.0 { write!(f, "The given index {i} is out of bounds.")
    } else { write!(f, "The given index is out of bounds.") }
}
impl_error! { individual: pub struct InvalidAxisLength(pub Option<usize>);
    DOC_INVALID_AXIS_LENGTH = "The given axis has an invalid length.\n\n
Optionally contains the given axis number.",
    self+f => if let Some(n) = self.0 {
        write!(f, "Axis number {n} has 0 length, which is not allowed.")
    } else { write!(f, "One ore more axis have 0 length, which is not allowed.") }
}
impl_error! { individual: pub struct KeyAlreadyExists;
    DOC_KEY_ALREADY_EXISTS = "The key already exists.",
    self+f => write!(f, "The key already exists.")
}
impl_error! { individual: pub struct MismatchedCapacity(pub Mismatch<usize, usize>);
    DOC_MISMATCHED_CAPACITY = "The given capacity did not match the required constraints.",
    self+f => write!(f, "Mismatched capacity: {:?}.", self.0)
}
impl_error! { individual: pub struct MismatchedDimensions(pub Mismatch<usize, usize>);
    DOC_MISMATCHED_DIMENSIONS = "The dimensions given did not match the elements provided.",
    self+f => write!(f, "Mismatched dimensions: {:?}.", self.0)
}
impl_error! { individual: pub struct MismatchedIndices;
    DOC_MISMATCHED_INDICES = "The given indices does not match the expected order.",
    self+f => write!(f, "The given indices does not match the expected order.")
}
impl_error! { individual: pub struct NodeEmpty(pub Option<usize>);
    DOC_NODE_EMPTY = "The node is empty.",
    self+f => if let Some(n) = self.0 { write!(f, "The given node `{n}` is empty.")
    } else { write!(f, "The node is empty.") }
}
impl_error! { individual: pub struct NodeLinkNotSet(pub Option<usize>);
    DOC_NODE_LINK_NOT_SET = "The link is not set.",
    self+f => if let Some(n) = self.0 { write!(f, "The given node link `{n}` is not set.")
    } else { write!(f, "The node link is not set.") }
}
impl_error! { individual: pub struct NodeLinkNotUnique(pub Option<usize>);
    DOC_NODE_LINK_NOT_UNIQUE = "The link is not unique.",
    self+f => if let Some(n) = self.0 { write!(f, "The given node link `{n}` is not unique.")
    } else { write!(f, "The node link is not unique.") }
}
impl_error! { individual: pub struct NotEnoughElements(pub Option<usize>);
    DOC_NOT_ENOUGH_ELEMENTS = "There are not enough elements for the operation.\n\n
Optionally contains the minimum number of elements needed.",
    self+f => if let Some(n) = self.0 {
        write!(f, "Not enough elements. Needs at least `{n}` elements.")
    } else { write!(f, "Not enough elements.") }
}
impl_error! { individual: pub struct NotEnoughSpace(pub Option<usize>);
    DOC_NOT_ENOUGH_SPACE = "There is not enough free space for the operation.\n\n
Optionally contains the number of free spaces needed.",
    self+f => if let Some(n) = self.0 {
        write!(f, "Not enough space. Needs at least `{n}` free space for elements.")
    } else { write!(f, "Not enough space.") }
}
impl_error! { individual: pub struct PartiallyAdded(pub Option<usize>);
    DOC_PARTIALLY_ADDED = "The operation could only add a subset of the elements.\n\n
Optionally contains the number of elements added.",
    self+f => if let Some(n) = self.0 { write!(f, "Only `{n}` elements could be added.")
    } else { write!(f, "Only a subset of elements could be added.") }
}

impl_error! { composite: fmt(f)
    /// An error composite of [`NotEnoughElements`] + [`NotEnoughSpace`].
    ///
    /// Used in methods of:
    /// [`Destaque`][crate::Destaque],
    /// [`Stack`][crate::Stack].
    pub enum DataNotEnough {
        DOC_NOT_ENOUGH_ELEMENTS:    Elements(i: Option<usize>)  => NotEnoughElements(*i),
        DOC_NOT_ENOUGH_SPACE:       Space(i: Option<usize>)     => NotEnoughSpace(*i),
    }
}
impl_error! { composite: fmt(f)
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
            DataOverflow(i: Option<usize>) => DataOverflow(*i),
        DOC_INDEX_OUT_OF_BOUNDS:
            IndexOutOfBounds(i: Option<usize>) => IndexOutOfBounds(*i),
        DOC_MISMATCHED_CAPACITY:
            MismatchedCapacity(c: Mismatch<usize, usize>) => MismatchedCapacity(*c),
        DOC_MISMATCHED_INDICES:
            MismatchedIndices => MismatchedIndices,
    }
}
// MAYBE: Merge with DataNotEnough
impl_error! { composite: fmt(f)
    /// An error composite of [`NotEnoughSpace`] + [`PartiallyAdded`].
    ///
    /// Used in methods of:
    /// [`ArrayUninit`][crate::ArrayUninit].
    pub enum PartialSpace {
        DOC_NOT_ENOUGH_SPACE:   NotEnoughSpace(i: Option<usize>) => NotEnoughSpace(*i),
        DOC_PARTIALLY_ADDED:    PartiallyAdded(i: Option<usize>) => PartiallyAdded(*i),
    }
}

#[cfg(all(feature = "error", data··))]
pub use full_composite::*;
#[cfg(all(feature = "error", data··))]
#[cfg_attr(feature = "nightly_doc", doc(cfg(all(feature = "error", data··))))]
mod full_composite {
    use super::*;
    use crate::{
        NotAvailable, NotImplemented, NotSupported, DOC_NOT_IMPLEMENTED, DOC_NOT_SUPPORTED,
    };

    impl_error! { composite: fmt(f)
        /// A data-related composite error.
        #[non_exhaustive]
        pub enum DataError {
            DOC_NOT_IMPLEMENTED: NotImplemented => NotImplemented,
            DOC_NOT_SUPPORTED: NotSupported => NotSupported,
            //
            DOC_DATA_OVERFLOW:
                DataOverflow => DataOverflow,
            DOC_ELEMENT_NOT_FOUND:
                ElementNotFound => ElementNotFound,
            DOC_INDEX_OUT_OF_BOUNDS:
                IndexOutOfBounds(i: Option<usize>) => IndexOutOfBounds(*i),
            DOC_INVALID_AXIS_LENGTH:
                InvalidAxisLength(i: Option<usize>) => InvalidAxisLength(*i),
            DOC_KEY_ALREADY_EXISTS:
                KeyAlreadyExists => KeyAlreadyExists,
            DOC_MISMATCHED_CAPACITY:
                MismatchedCapacity(c: Mismatch<usize, usize>) => MismatchedCapacity(*c),
            DOC_MISMATCHED_DIMENSIONS:
                MismatchedDimensions(d: Mismatch<usize, usize>) => MismatchedDimensions(*d),
            DOC_MISMATCHED_INDICES:
                MismatchedIndices => MismatchedIndices,
            DOC_NODE_EMPTY:
                NodeEmpty(i: Option<usize>) => NodeEmpty(*i),
            DOC_NODE_LINK_NOT_SET:
                NodeLinkNotSet(i: Option<usize>) => NodeLinkNotSet(*i),
            DOC_NODE_LINK_NOT_UNIQUE:
                NodeLinkNotUnique(i: Option<usize>) => NodeLinkNotUnique(*i),
            DOC_NOT_ENOUGH_ELEMENTS:
                NotEnoughElements(i: Option<usize>) => NotEnoughElements(*i),
            DOC_NOT_ENOUGH_SPACE:
                NotEnoughSpace(i: Option<usize>) => NotEnoughSpace(*i),
            DOC_PARTIALLY_ADDED:
                PartiallyAdded(i: Option<usize>) => PartiallyAdded(*i),
        }
    }
    impl_error! { composite: from(f): NotAvailable, for: DataError {
        NotImplemented => NotImplemented,
        NotSupported => NotSupported,
    }}
    impl_error! { composite: from(f): DataNotEnough, for: DataError {
        Elements(i) => NotEnoughElements(i),
        Space(i) => NotEnoughSpace(i),
    }}
    impl_error! { composite: from(f): PartialSpace, for: DataError {
        NotEnoughSpace(i) => NotEnoughSpace(i),
        PartiallyAdded(i) => PartiallyAdded(i),
    }}
    impl_error! { composite: from(f): MismatchedBounds, for: DataError {
        DataOverflow(i) => DataOverflow(i),
        IndexOutOfBounds(i) => IndexOutOfBounds(i),
        MismatchedIndices(i) => MismatchedIndices(i),
        MismatchedCapacity(i) => MismatchedCapacity(i),
    }}

    #[doc = crate::TAG_RESULT!()]
    /// A data-related result.
    pub type DataResult<T> = crate::Result<T, DataError>;
}
