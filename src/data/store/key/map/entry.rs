// devela/src/data/store/key/map/entry.rs
//
//! Defines [`StaticMapEntry`].
//

#[doc = crate::_tags!(data_structure)]
/// Represents an entry in a [static map] allowing for in-place mutation or insertion.
#[doc = crate::_doc_meta!{
    location("data/store/key", enum StaticMapEntry),
    #[cfg(target_pointer_width = "32")]
    test_size_of(__: StaticMapEntry<char> = 8|64; niche Option),
    #[cfg(target_pointer_width = "64")]
    test_size_of(__: StaticMapEntry<char> = 16|128; niche Option),
}]
///
/// [static map]: crate::map
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
