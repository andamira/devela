// devela/src/ui/view/view.rs
//
//! Defines [`UiViewFlags`], [`UiView`].
//

use crate::{LayoutRect, UiId, UiLayer};

crate::set! {
    #[doc = crate::_tags!(ui set)]
    /// Visual presentation flags of a UI view.
    #[doc = crate::_doc_meta! {
        location("ui/view"),
        test_size_of(UiViewFlags = 1),
    }]
    #[must_use]
    #[repr(transparent)]
    pub struct UiViewFlags(u8) {
        /// Not visually presented.
        HIDDEN = 0;

        /// Clips visual content to the view rectangle.
        CLIPPED = 1;

        /// Fully covers views behind it within its rectangle.
        OPAQUE = 2;
    }
}

#[doc = crate::_tags!(ui)]
/// Frame-local visual record for a UI identity.
#[doc = crate::_doc_meta! {
    location("ui/view"),
    #[cfg(target_pointer_width = "32")]
    test_size_of(UiView = 28|224),
    #[cfg(target_pointer_width = "64")]
    test_size_of(UiView = 32|256),
}]
///
/// Connects a resolved UI identity with the visual region and ordering layer
/// used for presentation.
#[must_use]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct UiView {
    id: UiId,
    rect: LayoutRect,
    layer: UiLayer,
    flags: UiViewFlags,
}
#[rustfmt::skip]
impl UiView {
    /* constructors */

    /// Constructs a visual record on the base layer with no flags.
    pub const fn new(id: UiId, rect: LayoutRect) -> Self {
        Self { id, rect, layer: UiLayer::BASE, flags: UiViewFlags::new() }
    }

    /// Constructs a visual record from all parts.
    pub const fn from_parts(id: UiId, rect: LayoutRect, layer: UiLayer, flags: UiViewFlags)
        -> Self { Self { id, rect, layer, flags } }

    /* queries */

    /// Returns the UI identity.
    #[must_use]
    pub const fn id(&self) -> UiId { self.id }

    /// Returns the visual rectangle.
    #[must_use]
    pub const fn rect(&self) -> LayoutRect { self.rect }

    /// Returns the visual ordering layer.
    #[must_use]
    pub const fn layer(&self) -> UiLayer { self.layer }

    /// Returns the visual flags.
    #[must_use]
    pub const fn flags(&self) -> UiViewFlags { self.flags }

    /// Returns whether the view is visually hidden.
    #[must_use]
    pub const fn is_hidden(&self) -> bool { self.flags.has(UiViewFlags::HIDDEN) }
    /// Returns whether the view is visually present.
    #[must_use]
    pub const fn is_visible(&self) -> bool { !self.is_hidden() }

    /* modifiers */

    /// Returns this view with another rectangle.
    #[must_use]
    pub const fn with_rect(self, rect: LayoutRect) -> Self { Self { rect, ..self } }

    /// Returns this view with another layer.
    #[must_use]
    pub const fn with_layer(self, layer: UiLayer) -> Self { Self { layer, ..self } }

    /// Returns this view with another flag set.
    #[must_use]
    pub const fn with_flags(self, flags: UiViewFlags) -> Self { Self { flags, ..self } }

    /// Returns this view with `flag` included.
    #[must_use]
    pub const fn with_flag(self, flag: UiViewFlags) -> Self {
        Self { flags: self.flags.with(flag), ..self }
    }
    /// Returns this view with `flag` removed.
    #[must_use]
    pub const fn without_flag(self, flag: UiViewFlags) -> Self {
        Self { flags: self.flags.without(flag), ..self }
    }
}
