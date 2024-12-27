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

impl_error! { individual: ElementNotFound,
    DOC_ELEMENT_NOT_FOUND = "The requested element has not been found.",
    self+f => write!(f, "The requested element has not been found."),
}
impl_error! { individual: InvalidAxisLength(pub Option<usize>),
    DOC_INVALID_AXIS_LENGTH = "The given axis has an invalid length.\n\n
Optionally contains some given axis number.",
    self+f => if let Some(n) = self.0 {
        write!(f, "Axis number {n} has 0 length, which is not allowed.")
    } else { write!(f, "One ore more axis have 0 length, which is not allowed.") }
}
impl_error! { individual: KeyAlreadyExists,
    DOC_KEY_ALREADY_EXISTS = "The key already exists.",
    self+f => write!(f, "The key already exists.")
}
impl_error! { individual: MismatchedDimensions(pub Mismatch<usize, usize>),
    DOC_MISMATCHED_DIMENSIONS = "The dimensions given did not match the elements provided.",
    self+f => write!(f, "Mismatched dimensions: {:?}.", self.0)
}
impl_error! { individual: MismatchedIndices,
    DOC_MISMATCHED_INDICES = "The given indices does not match the expected order.",
    self+f => write!(f, "The given indices does not match the expected order.")
}
impl_error! { individual: MismatchedLength(pub Mismatch<usize, usize>),
    DOC_MISMATCHED_LENGTH = "The given length or capacity did not match the required constraints.",
    self+f => write!(f, "Mismatched length or capacity: {:?}.", self.0)
}
impl_error! { individual: NodeEmpty(pub Option<usize>),
    DOC_NODE_EMPTY = "The node is empty.",
    self+f => if let Some(n) = self.0 { write!(f, "The given node `{n}` is empty.")
    } else { write!(f, "The node is empty.") }
}
impl_error! { individual: NodeLinkNotSet(pub Option<usize>),
    DOC_NODE_LINK_NOT_SET = "The link is not set.",
    self+f => if let Some(n) = self.0 { write!(f, "The given node link `{n}` is not set.")
    } else { write!(f, "The node link is not set.") }
}
impl_error! { individual: NodeLinkNotUnique(pub Option<usize>),
    DOC_NODE_LINK_NOT_UNIQUE = "The link is not unique.",
    self+f => if let Some(n) = self.0 { write!(f, "The given node link `{n}` is not unique.")
    } else { write!(f, "The node link is not unique.") }
}
impl_error! { individual: NotEnoughElements(pub Option<usize>),
    DOC_NOT_ENOUGH_ELEMENTS = "There are not enough elements for the operation.\n\n
Optionally contains the minimum number of elements needed.",
    self+f => if let Some(n) = self.0 {
        write!(f, "Not enough elements. Needs at least `{n}` elements.")
    } else { write!(f, "Not enough elements.") }
}
impl_error! { individual: NotEnoughSpace(pub Option<usize>),
    DOC_NOT_ENOUGH_SPACE = "There is not enough free space for the operation.\n\n
Optionally contains the number of free spaces needed.",
    self+f => if let Some(n) = self.0 {
        write!(f, "Not enough space. Needs at least `{n}` free space for elements.")
    } else { write!(f, "Not enough space.") }
}
impl_error! { individual: OutOfBounds(pub Option<usize>),
    DOC_OUT_OF_BOUNDS = "The given `index`, `length` or `capacity` is out of bounds.\n\n
Optionally contains some given magnitude.",
    self+f => if let Some(i) = self.0 { write!(f, "The given index {i} is out of bounds.")
    } else { write!(f, "The given index is out of bounds.") }
}
impl_error! { individual: Overflow,
    DOC_OVERFLOW = "Value above maximum representable.",
    self+f => write!(f, "Value above maximum representable.")
}
impl_error! { individual: PartiallyAdded(pub Option<usize>),
    DOC_PARTIALLY_ADDED = "The operation could only add a subset of the elements.\n\n
Optionally contains the number of elements added.",
    self+f => if let Some(n) = self.0 { write!(f, "Only `{n}` elements could be added.")
    } else { write!(f, "Only a subset of elements could be added.") }
}

#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "data")))]
#[cfg(feature = "data")]
mod full_composite {
    use super::*;
    use crate::{NotImplemented, NotSupported, DOC_NOT_IMPLEMENTED, DOC_NOT_SUPPORTED};

    #[doc = crate::TAG_RESULT!()]
    /// A data-related result.
    pub type DataResult<T> = crate::Result<T, DataError>;

    impl_error! { composite: fmt(f)
        /// A data-related composite error.
        #[non_exhaustive]
        pub enum DataError {
            DOC_NOT_IMPLEMENTED: NotImplemented => NotImplemented,
            DOC_NOT_SUPPORTED: NotSupported => NotSupported,
            //
            DOC_ELEMENT_NOT_FOUND: ElementNotFound => ElementNotFound,
            DOC_INVALID_AXIS_LENGTH:
                InvalidAxisLength(i: Option<usize>) => InvalidAxisLength(*i),
            DOC_KEY_ALREADY_EXISTS: KeyAlreadyExists => KeyAlreadyExists,
            DOC_MISMATCHED_DIMENSIONS:
                MismatchedDimensions(i: Mismatch<usize, usize>) => MismatchedDimensions(*i),
            DOC_MISMATCHED_INDICES: MismatchedIndices => MismatchedIndices,
            DOC_MISMATCHED_LENGTH:
                MismatchedLength(i: Mismatch<usize, usize>) => MismatchedLength(*i),
            DOC_NODE_EMPTY: NodeEmpty(i: Option<usize>) => NodeEmpty(*i),
            DOC_NODE_LINK_NOT_SET: NodeLinkNotSet(i: Option<usize>) => NodeLinkNotSet(*i),
            DOC_NODE_LINK_NOT_UNIQUE: NodeLinkNotUnique(i: Option<usize>) => NodeLinkNotUnique(*i),
            DOC_NOT_ENOUGH_ELEMENTS:
                NotEnoughElements(i: Option<usize>) => NotEnoughElements(*i),
            DOC_NOT_ENOUGH_SPACE: NotEnoughSpace(i: Option<usize>) => NotEnoughSpace(*i),
            DOC_OUT_OF_BOUNDS: OutOfBounds(i: Option<usize>) => OutOfBounds(*i),
            DOC_OVERFLOW: Overflow => Overflow,
            DOC_PARTIALLY_ADDED: PartiallyAdded(i: Option<usize>) => PartiallyAdded(*i),
        }
    }
    impl_error! { composite: from(f): DataNotEnough, for: DataError {
        Elements(i) => NotEnoughElements(i),
        Space(i) => NotEnoughSpace(i),
    }}
    impl_error! { composite: from(f): PartialSpace, for: DataError {
        NotEnoughSpace(i) => NotEnoughSpace(i),
        PartiallyAdded(i) => PartiallyAdded(i),
    }}

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

impl_error! { composite: fmt(f)
    /// An error composite of [`NotEnoughElements`] + [`NotEnoughSpace`].
    ///
    /// Used in methods of:
    /// - [`Destaque`][crate::Destaque].
    pub enum DataNotEnough {
        DOC_NOT_ENOUGH_ELEMENTS:    Elements(i: Option<usize>)  => NotEnoughElements(*i),
        DOC_NOT_ENOUGH_SPACE:       Space(i: Option<usize>)     => NotEnoughSpace(*i),
    }
}
impl_error! { composite: fmt(f)
    /// An error composite of [`NotEnoughSpace`] + [`PartiallyAdded`].
    ///
    /// Used in methods of:
    /// - [`ArrayUninit`][crate::ArrayUninit].
    pub enum PartialSpace {
        DOC_NOT_ENOUGH_SPACE:   NotEnoughSpace(i: Option<usize>) => NotEnoughSpace(*i),
        DOC_PARTIALLY_ADDED:    PartiallyAdded(i: Option<usize>) => PartiallyAdded(*i),
    }
}
