// devela/src/ui/view/form/view.rs
//
//! Defines [`UiViewForm`].
//

#[doc = crate::_tags!(ui)]
/// Projection form of a UI view.
#[doc = crate::_doc_meta! {
    location("ui/view"),
    test_size_of(UiViewForm = 1|8),
}]
///
/// Describes the broad presentation form a UI view is intended for,
/// without selecting a concrete backend.
#[must_use]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum UiViewForm {
    /// Cell-oriented presentation.
    ///
    /// Suitable for terminals, text grids, tile grids,
    /// and other discrete-cell output spaces.
    #[default]
    Cell,

    /// Graphic presentation.
    ///
    /// Suitable for pixel, vector, canvas, framebuffer, and GPU-backed output.
    Graphic,

    /// Document presentation.
    ///
    /// Suitable for structured document output such as HTML,
    /// rich text, or paged/exported representations.
    Document,

    /// Message-oriented presentation.
    ///
    /// Suitable for conversational interfaces, chat bots, REPLs, command
    /// exchanges, and turn-based textual adventures.
    Message,
}
