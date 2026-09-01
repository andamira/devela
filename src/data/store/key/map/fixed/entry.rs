// devela/src/data/store/key/map/fixed/entry.rs
//
//! Defines [`MapFixedEntry`].
//

#[doc = crate::_tags!(data_structure)]
/// Represents an entry in a [fixed-capacity map`],
/// allowing in-place mutation or insertion.
#[doc = crate::_doc_meta!{
    location("data/store/key/map", enum MapFixedEntry),
    #[cfg(target_pointer_width = "32")]
    test_size_of(MapFixedEntry<char> = 8|64; niche Option),
    #[cfg(target_pointer_width = "64")]
    test_size_of(MapFixedEntry<char> = 16|128; niche Option),
}]
/// [fixed-capacity map]: crate::map!
#[derive(Debug)]
pub enum MapFixedEntry<'a, V> {
    /// An entry that contains a value.
    ///
    /// Provides a mutable reference to the stored value, allowing in-place modification.
    Occupied(&'a mut V),
    /// An entry that is vacant and can be used for insertion.
    ///
    /// Stores the index where a new value should be inserted.
    Vacant(usize),
}
