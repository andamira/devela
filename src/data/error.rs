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
// - aggregated Data-related error types:
//   - DataResult
//   - DataError

use crate::{impl_error, Mismatch};

impl_error![define: ErrorElementNotFound,
    DOC_ERROR_ELEMENT_NOT_FOUND = "The requested element has not been found.",
    self+f => write!(f, "The requested element has not been found."),
];
impl_error![define: ErrorInvalidAxisLength(pub Option<usize>),
    DOC_ERROR_INVALID_AXIS_LENGTH = "The given axis has an invalid length.\n\n
Optionally contains some given axis number.",
    self+f => if let Some(n) = self.0 {
        write!(f, "Axis number {n} has 0 length, which is not allowed.")
    } else { write!(f, "One ore more axis have 0 length, which is not allowed.") }
];
impl_error![define: ErrorKeyAlreadyExists,
    DOC_ERROR_KEY_ALREADY_EXISTS = "The key already exists.",
    self+f => write!(f, "The key already exists.")
];
impl_error![define: ErrorMismatchedDimensions(pub Mismatch<usize, usize>),
    DOC_ERROR_MISMATCHED_DIMENSIONS = "The dimensions given did not match the elements provided.",
    self+f => write!(f, "Mismatched dimensions: {:?}.", self.0)
];
impl_error![define: ErrorMismatchedIndices,
    DOC_ERROR_MISMATCHED_INDICES = "The given indices does not match the expected order.",
    self+f => write!(f, "The given indices does not match the expected order.")
];
impl_error![define: ErrorMismatchedLength(pub Mismatch<usize, usize>),
    DOC_ERROR_MISMATCHED_LENGTH =
    "The given length or capacity did not match the required constraints.",
    self+f => write!(f, "Mismatched length or capacity: {:?}.", self.0)
];
impl_error![define: ErrorNodeEmpty(pub Option<usize>),
    DOC_ERROR_NODE_EMPTY = "The node is empty.",
    self+f => if let Some(n) = self.0 { write!(f, "The given node `{n}` is empty.")
    } else { write!(f, "The node is empty.") }
];
impl_error![define: ErrorNodeLinkNotSet(pub Option<usize>),
    DOC_ERROR_NODE_LINK_NOT_SET = "The link is not set.",
    self+f => if let Some(n) = self.0 { write!(f, "The given node link `{n}` is not set.")
    } else { write!(f, "The node link is not set.") }
];
impl_error![define: ErrorNodeLinkNotUnique(pub Option<usize>),
    DOC_ERROR_NODE_LINK_NOT_UNIQUE = "The link is not unique.",
    self+f => if let Some(n) = self.0 { write!(f, "The given node link `{n}` is not unique.")
    } else { write!(f, "The node link is not unique.") }
];
impl_error![define: ErrorNotEnoughElements(pub Option<usize>),
    DOC_ERROR_NOT_ENOUGH_ELEMENTS = "There are not enough elements for the operation.\n\n
Optionally contains the minimum number of elements needed.",
    self+f => if let Some(n) = self.0 {
        write!(f, "Not enough elements. Needs at least `{n}` elements.")
    } else { write!(f, "Not enough elements.") }
];
impl_error![define: ErrorNotEnoughSpace(pub Option<usize>),
    DOC_ERROR_NOT_ENOUGH_SPACE = "There is not enough free space for the operation.\n\n
Optionally contains the number of free spaces needed.",
    self+f => if let Some(n) = self.0 {
        write!(f, "Not enough space. Needs at least `{n}` free space for elements.")
    } else { write!(f, "Not enough space.") }
];
impl_error![define: ErrorOutOfBounds(pub Option<usize>),
    DOC_ERROR_OUT_OF_BOUNDS = "The given `index`, `length` or `capacity` is out of bounds.\n\n
Optionally contains some given magnitude.",
    self+f => if let Some(i) = self.0 { write!(f, "The given index {i} is out of bounds.")
    } else { write!(f, "The given index is out of bounds.") }
];
impl_error![define: ErrorOverflow,
    DOC_ERROR_OVERFLOW = "Value above maximum representable.",
    self+f => write!(f, "Value above maximum representable.")
];
impl_error![define: ErrorPartiallyAdded(pub Option<usize>),
    DOC_ERROR_PARTIALLY_ADDED = "The operation could only add a subset of the elements.\n\n
Optionally contains the number of elements added.",
    self+f => if let Some(n) = self.0 { write!(f, "Only `{n}` elements could be added.")
    } else { write!(f, "Only a subset of elements could be added.") }
];

#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "data")))]
#[cfg(feature = "data")]
mod aggregated {
    use super::*;
    use crate::{
        Display, ErrorNotImplemented, ErrorNotSupported, FmtResult, Formatter,
        DOC_ERROR_NOT_IMPLEMENTED, DOC_ERROR_NOT_SUPPORTED,
    };

    /// A data-related aggregated result.
    pub type DataResult<T> = crate::Result<T, DataError>;

    /// A data-related aggregated error.
    #[non_exhaustive] #[rustfmt::skip]
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub enum DataError {
        #[doc = DOC_ERROR_NOT_IMPLEMENTED!()]       NotImplemented,
        #[doc = DOC_ERROR_NOT_SUPPORTED!()]         NotSupported,
        //
        #[doc = DOC_ERROR_ELEMENT_NOT_FOUND!()]     ElementNotFound,
        #[doc = DOC_ERROR_INVALID_AXIS_LENGTH!()]   InvalidAxisLength(Option<usize>),
        #[doc = DOC_ERROR_KEY_ALREADY_EXISTS!()]    KeyAlreadyExists,
        #[doc = DOC_ERROR_MISMATCHED_DIMENSIONS!()] MismatchedDimensions(Mismatch<usize, usize>),
        #[doc = DOC_ERROR_MISMATCHED_INDICES!()]    MismatchedIndices,
        #[doc = DOC_ERROR_MISMATCHED_LENGTH!()]     MismatchedLength(Mismatch<usize, usize>),
        #[doc = DOC_ERROR_NODE_EMPTY!()]            NodeEmpty(Option<usize>),
        #[doc = DOC_ERROR_NODE_LINK_NOT_SET!()]     NodeLinkNotSet(Option<usize>),
        #[doc = DOC_ERROR_NODE_LINK_NOT_UNIQUE!()]  NodeLinkNotUnique(Option<usize>),
        #[doc = DOC_ERROR_NOT_ENOUGH_ELEMENTS!()]   NotEnoughElements(Option<usize>),
        #[doc = DOC_ERROR_NOT_ENOUGH_SPACE!()]      NotEnoughSpace(Option<usize>),
        #[doc = DOC_ERROR_OUT_OF_BOUNDS!()]         OutOfBounds(Option<usize>),
        #[doc = DOC_ERROR_OVERFLOW!()]              Overflow,
        #[doc = DOC_ERROR_PARTIALLY_ADDED!()]       PartiallyAdded(Option<usize>),
    }
    impl crate::Error for DataError {}
    impl crate::ExtError for DataError {
        type Kind = (); // TODO
        fn error_eq(&self, other: &Self) -> bool {
            self == other
        }
        fn error_kind(&self) -> Self::Kind {}
    }
    impl Display for DataError {
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult<()> {
            use super::DataError as E;
            match self {
                E::NotImplemented => Display::fmt(&ErrorNotImplemented, f),
                E::NotSupported => Display::fmt(&ErrorNotSupported, f),
                //
                E::ElementNotFound => Display::fmt(&ErrorElementNotFound, f),
                E::InvalidAxisLength(n) => Display::fmt(&ErrorInvalidAxisLength(*n), f),
                E::KeyAlreadyExists => Display::fmt(&ErrorKeyAlreadyExists, f),
                E::MismatchedDimensions(m) => Display::fmt(&ErrorMismatchedDimensions(*m), f),
                E::MismatchedIndices => Display::fmt(&ErrorMismatchedIndices, f),
                E::MismatchedLength(l) => Display::fmt(&ErrorMismatchedLength(*l), f),
                E::NodeEmpty(n) => Display::fmt(&ErrorNodeEmpty(*n), f),
                E::NodeLinkNotSet(n) => Display::fmt(&ErrorNodeLinkNotSet(*n), f),
                E::NodeLinkNotUnique(n) => Display::fmt(&ErrorNodeLinkNotUnique(*n), f),
                E::NotEnoughElements(n) => Display::fmt(&ErrorNotEnoughElements(*n), f),
                E::NotEnoughSpace(n) => Display::fmt(&ErrorNotEnoughSpace(*n), f),
                E::OutOfBounds(i) => Display::fmt(&ErrorOutOfBounds(*i), f),
                E::Overflow => Display::fmt(&ErrorOverflow, f),
                E::PartiallyAdded(n) => Display::fmt(&ErrorPartiallyAdded(*n), f),
            }
        }
    }

    impl_error! { for: DataError, from: {
        ErrorNotImplemented,        _f => DataError::NotImplemented,
        ErrorNotSupported,          _f => DataError::NotSupported,
        //
        ErrorElementNotFound,       _f => DataError::ElementNotFound,
        ErrorInvalidAxisLength,      f => DataError::InvalidAxisLength(f.0),
        ErrorKeyAlreadyExists,      _f => DataError::KeyAlreadyExists,
        ErrorMismatchedDimensions,   f => DataError::MismatchedDimensions(f.0),
        ErrorMismatchedIndices,     _f => DataError::MismatchedIndices,
        ErrorMismatchedLength,       f => DataError::MismatchedLength(f.0),
        ErrorNodeEmpty,              f => DataError::NodeEmpty(f.0),
        ErrorNodeLinkNotSet,         f => DataError::NodeLinkNotSet(f.0),
        ErrorNodeLinkNotUnique,      f => DataError::NodeLinkNotUnique(f.0),
        ErrorNotEnoughElements,      f => DataError::NotEnoughElements(f.0),
        ErrorNotEnoughSpace,         f => DataError::NotEnoughSpace(f.0),
        ErrorOutOfBounds,            f => DataError::OutOfBounds(f.0),
        ErrorOverflow,              _f => DataError::Overflow,
        ErrorPartiallyAdded,         f => DataError::PartiallyAdded(f.0),
    }}

    // RETHINK
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
pub use aggregated::*;
