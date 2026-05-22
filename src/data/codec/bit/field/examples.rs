// devela::data::codec::bit::field::examples
//
//! Defines [`BitfieldExample`]
//

crate::bitfield! {
    #[doc = crate::_tags!(example bit data_structure)]
    /// A compact packet header.
    #[doc = crate::_doc_location!("data/codec")]
    ///
    /// It has been generated with the [`bitfield!`] macro like this:
    /// ```
    /// # use devela::bitfield;
    /// bitfield! {
    ///     /// A compact packet header.
    ///     pub struct BitfieldExample(u16) {
    ///         KIND    = 0..=3;
    ///         VERSION = 4..=7;
    ///         LENGTH  = 8..=15;
    ///     }
    /// }
    /// ```
    pub struct BitfieldExample(u16) {
        KIND    = 0..=3;
        VERSION = 4..=7;
        LENGTH  = 8..=15;
    }
}
