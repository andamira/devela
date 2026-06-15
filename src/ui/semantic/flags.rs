// devela/src/ui/flags.rs
//
//! Defines [`UiFlags`].
//

crate::set! {
    #[doc = crate::_tags!(ui set)]
    /// Semantic state flags of a UI identity.
    #[doc = crate::_doc_meta! {
        location("ui/semantic"),
        test_size_of(UiFlags = 2|16),
    }]
    #[must_use]
    #[repr(transparent)]
    pub struct UiFlags(u16) {
        DISABLED = 0;
        READ_ONLY = 1;
        REQUIRED = 2;
        CHECKED = 3;
        SELECTED = 4;
        EXPANDED = 5;
        PRESSED = 6;
        BUSY = 7;
        HIDDEN = 8;
    }
}
