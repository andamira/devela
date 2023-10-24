// devela::data::trait
//
//!
//

/// Represents an abstract data collection.
pub trait DataCollection {
    /// The element type of the collection.
    type Element;

    /// Returns `true` if the collection is empty, `false` if it's not,
    /// `None` if that's not well defined for this particular collection.
    fn collection_is_empty(&self) -> Option<bool>;

    /// Returns `true` if the collection is full, `false` if it's not,
    /// `None` if that's not well defined for this particular collection.
    fn collection_is_full(&self) -> Option<bool>;

    /// Returns the reserved capacity for elements in the collection.
    fn collection_capacity(&self) -> usize;

    /// Returns the current number of elements in the collection.
    fn collection_len(&self) -> usize;
}
