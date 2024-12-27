// devela::data::error
//
//! Data-related error types.
//
// TOC
// - standalone data-related error types:
//   - ElementNotFound
//   - InvalidAxisLength
//   - KeyAlreadyExists
//   - MismatchedDimensions
//   - MismatchedIndices
//   - MismatchedLength
//   - NodeEmpty
//   - NodeLinkNotSet
//   - NodeLinkNotUnique
//   - NotEnoughElements
//   - NotEnoughSpace
//   - OutOfBounds
//   - Overflow
//   - PartiallyAdded
// - full composite errors:
//   - DataResult
//   - DataError
// - partial composite errors:
//   - PartialSpace

use crate::{impl_error, Mismatch};

impl_error![single: ElementNotFound,
    DOC_ERROR_ELEMENT_NOT_FOUND = "The requested element has not been found.",
    self+f => write!(f, "The requested element has not been found."),
];
impl_error![single: InvalidAxisLength(pub Option<usize>),
    DOC_ERROR_INVALID_AXIS_LENGTH = "The given axis has an invalid length.\n\n
Optionally contains some given axis number.",
    self+f => if let Some(n) = self.0 {
        write!(f, "Axis number {n} has 0 length, which is not allowed.")
    } else { write!(f, "One ore more axis have 0 length, which is not allowed.") }
];
impl_error![single: KeyAlreadyExists,
    DOC_ERROR_KEY_ALREADY_EXISTS = "The key already exists.",
    self+f => write!(f, "The key already exists.")
];
impl_error![single: MismatchedDimensions(pub Mismatch<usize, usize>),
    DOC_ERROR_MISMATCHED_DIMENSIONS = "The dimensions given did not match the elements provided.",
    self+f => write!(f, "Mismatched dimensions: {:?}.", self.0)
];
impl_error![single: MismatchedIndices,
    DOC_ERROR_MISMATCHED_INDICES = "The given indices does not match the expected order.",
    self+f => write!(f, "The given indices does not match the expected order.")
];
impl_error![single: MismatchedLength(pub Mismatch<usize, usize>),
    DOC_ERROR_MISMATCHED_LENGTH =
    "The given length or capacity did not match the required constraints.",
    self+f => write!(f, "Mismatched length or capacity: {:?}.", self.0)
];
impl_error![single: NodeEmpty(pub Option<usize>),
    DOC_ERROR_NODE_EMPTY = "The node is empty.",
    self+f => if let Some(n) = self.0 { write!(f, "The given node `{n}` is empty.")
    } else { write!(f, "The node is empty.") }
];
impl_error![single: NodeLinkNotSet(pub Option<usize>),
    DOC_ERROR_NODE_LINK_NOT_SET = "The link is not set.",
    self+f => if let Some(n) = self.0 { write!(f, "The given node link `{n}` is not set.")
    } else { write!(f, "The node link is not set.") }
];
impl_error![single: NodeLinkNotUnique(pub Option<usize>),
    DOC_ERROR_NODE_LINK_NOT_UNIQUE = "The link is not unique.",
    self+f => if let Some(n) = self.0 { write!(f, "The given node link `{n}` is not unique.")
    } else { write!(f, "The node link is not unique.") }
];
impl_error![single: NotEnoughElements(pub Option<usize>),
    DOC_ERROR_NOT_ENOUGH_ELEMENTS = "There are not enough elements for the operation.\n\n
Optionally contains the minimum number of elements needed.",
    self+f => if let Some(n) = self.0 {
        write!(f, "Not enough elements. Needs at least `{n}` elements.")
    } else { write!(f, "Not enough elements.") }
];
impl_error![single: NotEnoughSpace(pub Option<usize>),
    DOC_ERROR_NOT_ENOUGH_SPACE = "There is not enough free space for the operation.\n\n
Optionally contains the number of free spaces needed.",
    self+f => if let Some(n) = self.0 {
        write!(f, "Not enough space. Needs at least `{n}` free space for elements.")
    } else { write!(f, "Not enough space.") }
];
impl_error![single: OutOfBounds(pub Option<usize>),
    DOC_ERROR_OUT_OF_BOUNDS = "The given `index`, `length` or `capacity` is out of bounds.\n\n
Optionally contains some given magnitude.",
    self+f => if let Some(i) = self.0 { write!(f, "The given index {i} is out of bounds.")
    } else { write!(f, "The given index is out of bounds.") }
];
impl_error![single: Overflow,
    DOC_ERROR_OVERFLOW = "Value above maximum representable.",
    self+f => write!(f, "Value above maximum representable.")
];
impl_error![single: PartiallyAdded(pub Option<usize>),
    DOC_ERROR_PARTIALLY_ADDED = "The operation could only add a subset of the elements.\n\n
Optionally contains the number of elements added.",
    self+f => if let Some(n) = self.0 { write!(f, "Only `{n}` elements could be added.")
    } else { write!(f, "Only a subset of elements could be added.") }
];

#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "data")))]
#[cfg(feature = "data")]
mod full_composite {
    use super::*;
    use crate::{NotImplemented, NotSupported, DOC_ERROR_NOT_IMPLEMENTED, DOC_ERROR_NOT_SUPPORTED};

    #[doc = crate::TAG_RESULT!()]
    /// A data-related result.
    pub type DataResult<T> = crate::Result<T, DataError>;

    impl_error! { composite:
        /// A data-related composite error.
        #[non_exhaustive]
        pub enum DataError {
            DOC_ERROR_NOT_IMPLEMENTED:       NotImplemented,
            DOC_ERROR_NOT_SUPPORTED:         NotSupported,
            //
            DOC_ERROR_ELEMENT_NOT_FOUND:     ElementNotFound,
            DOC_ERROR_INVALID_AXIS_LENGTH:   InvalidAxisLength(i: Option<usize>),
            DOC_ERROR_KEY_ALREADY_EXISTS:    KeyAlreadyExists,
            DOC_ERROR_MISMATCHED_DIMENSIONS: MismatchedDimensions(i: Mismatch<usize, usize>),
            DOC_ERROR_MISMATCHED_INDICES:    MismatchedIndices,
            DOC_ERROR_MISMATCHED_LENGTH:     MismatchedLength(i: Mismatch<usize, usize>),
            DOC_ERROR_NODE_EMPTY:            NodeEmpty(i: Option<usize>),
            DOC_ERROR_NODE_LINK_NOT_SET:     NodeLinkNotSet(i: Option<usize>),
            DOC_ERROR_NODE_LINK_NOT_UNIQUE:  NodeLinkNotUnique(i: Option<usize>),
            DOC_ERROR_NOT_ENOUGH_ELEMENTS:   NotEnoughElements(i: Option<usize>),
            DOC_ERROR_NOT_ENOUGH_SPACE:      NotEnoughSpace(i: Option<usize>),
            DOC_ERROR_OUT_OF_BOUNDS:         OutOfBounds(i: Option<usize>),
            DOC_ERROR_OVERFLOW:              Overflow,
            DOC_ERROR_PARTIALLY_ADDED:       PartiallyAdded(i: Option<usize>),
        }
        [fmt]
    }

    #[allow(dead_code)]
    impl DataError {
        pub(crate) const fn ni<T>() -> DataResult<T> {
            Err(DataError::NotImplemented)
        }
        pub(crate) const fn ns<T>() -> DataResult<T> {
            Err(DataError::NotSupported)
        }
    }
}
#[cfg(feature = "data")]
pub use full_composite::*;

/* Partial Composite Errors */

impl_error! { composite:
    /// An error composite of
    /// [`NotEnoughSpace`], [`PartiallyAdded`].
    ///
    /// Used in methods of:
    /// - [`ArrayUninit`][crate::ArrayUninit].
    pub enum PartialSpace {
        DOC_ERROR_NOT_ENOUGH_SPACE: NotEnoughSpace(i: Option<usize>),
        DOC_ERROR_PARTIALLY_ADDED: PartiallyAdded(i: Option<usize>),
    }
    [fmt]
}
