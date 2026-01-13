// devela_base_core::data::error
//
//! Data-related errors.
//
// TOC
// - individual data-related error types:
//   - CapacityMismatch
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
//   - MismatchedBounds: CapacityMismatch, IndexOutOfBounds, MismatchedIndices
//   - PartialSpace:     NotEnoughSpace, PartiallyAdded

use crate::{_TAG_DATA, Boundary1d, Mismatch, define_error};

/* individual errors */

define_error! { individual:
    /// Represents an absolute mismatch: the operation cannot succeed
    /// for the given value, regardless of the current state or any retry.
    ///
    /// # Examples
    /// ```
    /// # use devela_base_core::{Boundary1d, CapacityMismatch};
    /// // Upper-bound violation. Capacity is fixed at 8, value tried was 12:
    /// let err = CapacityMismatch::too_large(12, 8);
    /// assert_eq!(err.bound, Boundary1d::Upper);
    /// assert_eq!(err.excess(), Some(4));
    /// assert_eq!(err.missing(), None);
    ///
    /// // Lower-bound violation. At least 16 bytes required, only 10 provided:
    /// let err = CapacityMismatch::too_small(10, 16);
    /// assert_eq!(err.bound, Boundary1d::Lower);
    /// assert_eq!(err.missing(), Some(6));
    /// assert_eq!(err.excess(), None);
    ///
    /// // Direction known, quantities unknown:
    /// let err = CapacityMismatch::none(Boundary1d::Upper);
    /// assert_eq!(err.excess(), None);
    /// assert_eq!(err.missing(), None);
    /// ```
    pub struct CapacityMismatch {
        /// Which boundary of the capacity constraint applies.
        pub bound: Boundary1d,
        /// The value involved in the capacity check, if known.
        pub value: Option<usize>,
        /// The capacity limit involved in the check, if known.
        pub limit: Option<usize>,
    }
    +location: "data/error",
    +tag: _TAG_DATA!(),
    DOC_CAPACITY_MISMATCH = "The operation did not satisfy a finite capacity constraint.",
    self+f => match (self.bound, self.value, self.limit) {
        (Boundary1d::Upper, Some(v), Some(l)) =>
            write!(f, "The value {v} exceeded the available capacity {l}."),
        (Boundary1d::Upper, Some(v), None) =>
            write!(f, "The value {v} exceeded an available finite capacity."),
        (Boundary1d::Upper, None, Some(l)) =>
            write!(f, "The operation exceeded the available capacity {l}."),

        (Boundary1d::Lower, Some(v), Some(l)) =>
            write!(f, "The value {v} was insufficient for the required capacity {l}."),
        (Boundary1d::Lower, Some(v), None) =>
            write!(f, "The value {v} was insufficient for the required capacity."),
        (Boundary1d::Lower, None, Some(l)) =>
            write!(f, "The operation did not meet the required capacity {l}."),
        _ => f.write_str(DOC_CAPACITY_MISMATCH!())
    }
}
/// Constructors
#[rustfmt::skip]
impl CapacityMismatch {
    /// Creates a capacity mismatch with no additional information.
    pub const fn none(bound: Boundary1d) -> Self {
        Self { bound, value: None, limit: None }
    }
    /// Creates a mismatch with a known value.
    pub const fn value(bound: Boundary1d, value: usize) -> Self {
        Self { bound, value: Some(value), limit: None }
    }
    /// Creates a mismatch with a known limit.
    pub const fn limit(bound: Boundary1d, limit: usize) -> Self {
        Self { bound, value: None, limit: Some(limit) }
    }
    /// Creates a mismatch with both value and limit known.
    pub const fn value_limit(bound: Boundary1d, value: usize, limit: usize) -> Self {
        Self { bound, value: Some(value), limit: Some(limit) }
    }
    /// Creates a capacity mismatch indicating that a value was
    /// **below** the required capacity.
    ///
    /// This corresponds to a violation of the **lower** boundary.
    pub const fn too_small(value: usize, limit: usize) -> Self {
        Self::value_limit(Boundary1d::Lower, value, limit)
    }
    /// Creates a capacity mismatch indicating that a value
    /// **exceeded** an available capacity.
    ///
    /// This corresponds to a violation of the **upper** boundary.
    pub const fn too_large(value: usize, limit: usize) -> Self {
        Self::value_limit(Boundary1d::Upper, value, limit)
    }
    /// Creates an upper-bound capacity mismatch from a fallible value computation.
    ///
    /// If the value is available, it is recorded.
    /// Otherwise, the mismatch is created without a concrete value.
    pub const fn too_large_try<E: Copy>(value: Result<usize, E>, limit: usize) -> Self {
        match value {
            Ok(v) => Self::too_large(v, limit),
            Err(_) => Self::limit(Boundary1d::Upper, limit),
        }
    }
}
/// Queries
impl CapacityMismatch {
    /// Returns the amount by which the value exceeded the limit,
    /// if this is an upper-bound mismatch and both quantities are known.
    pub const fn excess(&self) -> Option<usize> {
        match (self.bound, self.value, self.limit) {
            (Boundary1d::Upper, Some(v), Some(l)) if v > l => Some(v - l),
            _ => None,
        }
    }
    /// Returns the amount by which the value fell short of the limit,
    /// if this is a lower-bound mismatch and both quantities are known.
    pub const fn missing(&self) -> Option<usize> {
        match (self.bound, self.value, self.limit) {
            (Boundary1d::Lower, Some(v), Some(l)) if v < l => Some(l - v),
            _ => None,
        }
    }
}
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
    /// [`CapacityMismatch`] + [`IndexOutOfBounds`] + [`MismatchedIndices`]
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
        DOC_CAPACITY_MISMATCH: +const CapacityMismatch {
            /// Which boundary of the capacity constraint applies.
            bound: Boundary1d,
            /// The value involved in the capacity check, if known.
            value: Option<usize>,
            /// The capacity limit involved in the check, if known.
            limit: Option<usize>
        } => CapacityMismatch { bound: *bound, value: *value, limit: *limit },

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
