// devela/src/num/grain/lim/_example.rs
//
//! Defines [`BoundI8Example`].
//

use crate::bound_int;

bound_int! {
    #[doc = crate::_tags!(example num)]
    /// Example bounded integer with embedded boundary metadata created with [`bound_int!`].
    #[doc = crate::_doc_meta!{
        location("num/grain"),
        test_size_of(BoundI8Example = 1|8; niche Option),
    }]
    /// This compact `i8` example uses 5 bits for the signed payload and the
    /// remaining 3 bits for metadata: 2 count bits plus 1 direction bit.
    ///
    /// The payload range is `-16..=15`, and the boundary count saturates at
    /// [`MAX_COUNT`](#associatedconstant.MAX_COUNT).
    ///
    /// # Methods
    /// TODO
    pub struct BoundI8Example: repr(crate::NonMinI8 => i8);

    value_bits(8-3);
    ops(all);
}

crate::bound_int! {
    #[doc = crate::_tags!(example num)]
    /// Example bounded integer with a symmetric signed payload range.
    #[doc = crate::_doc_meta!{ location("num/grain") }]
    pub struct BoundI8SymExample: repr(crate::NonMinI8 => i8);

    value_bits(8-3);
    range(symmetric);
    ops(all);
}
