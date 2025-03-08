// devela::data::key::static_map::entry
//
//!
//

/// Represents an entry in a [static map] allowing for in-place mutation or insertion.
///
/// [static map]: crate::define_static_map
#[derive(Debug)]
pub enum StaticMapEntry<'a, V> {
    /// An entry that contains a value.
    ///
    /// Provides a mutable reference to the stored value, allowing in-place modification.
    Occupied(&'a mut V),
    /// An entry that is vacant and can be used for insertion.
    ///
    /// Stores the index where a new value should be inserted.
    Vacant(usize),
}
