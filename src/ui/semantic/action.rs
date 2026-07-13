// devela/src/ui/action.rs
//
//! Defines [`UiAction`].
//

crate::enumset! {

    #[doc = crate::_tags!(ui interaction member)]
    /// Human-facing action exposed by a UI identity.
    #[doc = crate::_doc_meta! {location("ui/semantic")}]
    #[must_use]
    pub enum UiAction(

        #[doc = crate::_tags!(ui set)]
        /// Set of semantic actions exposed by a UI identity.
        #[doc = crate::_doc_meta! {
            location("ui/semantic"),
            test_size_of(UiActions = 2|16),
        }]
        #[must_use]
        #[repr(transparent)]
        pub UiActions: u16

    ) {
        /// Activate the item.
        Activate,

        /// Move focus to the item.
        Focus,

        /// Select the item.
        Select,

        /// Toggle the item.
        Toggle,

        /// Increase the current value.
        Increment,

        /// Decrease the current value.
        Decrement,

        /// Expand hidden or collapsed content.
        Expand,

        /// Collapse visible expanded content.
        Collapse,

        /// Open a resource, view, or container.
        Open,

        /// Close a resource, view, or container.
        Close,
    }
}
