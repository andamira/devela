// devela::collections::trait
//
//!
//

/// A trait for an abstract collection.
pub trait Collection {
    /// The element type of the collection.
    type Element;

    /// Returns `true` if the collection is empty, `false` if it's not,
    /// `None` if it's not well defined for this data strucure.
    fn collection_is_empty(&self) -> Option<bool>;

    /// Returns `true` if the collection is full, `false` if it's not,
    /// and `None` if it's not well defined for this data structure.
    fn collection_is_full(&self) -> Option<bool>;

    /// Returns the current total capacity for elements of the collection.
    fn collection_capacity(&self) -> usize;

    /// Returns the number of elements in the collection.
    fn collection_len(&self) -> usize;
}

// MAYBE
// /// An abstract dynamically-sized Collection.
// pub trait DynDataCollection: DataCollection {
//     fn collection_with_capacity(capacity: usize) -> Self;
//     fn collection_capacity(&self) -> usize;
//     fn collection_set_capacity(&mut self, capacity: usize) -> Result<()>;
//     //
//     fn collection_remaining_capacity(&self) -> usize {
//         self.collection_capacity() - self.collection_len()
//     }
// }
