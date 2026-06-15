// devela/src/ui/view/layer.rs
//
//! Defines [`UiLayer`].
//

#[doc = crate::_tags!(ui)]
/// Visual ordering layer for a UI view.
#[doc = crate::_doc_meta! {
    location("ui/view"),
    test_size_of(UiLayer = 2|16),
}]
#[must_use]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct UiLayer(i16);

#[rustfmt::skip]
impl UiLayer {
    /// The default visual layer.
    pub const BASE: Self = Self(0);

    /// Constructs a visual layer from its raw value.
    pub const fn new(raw: i16) -> Self { Self(raw) }

    /// Returns the raw layer value.
    #[must_use]
    pub const fn raw(self) -> i16 { self.0 }

    /// Returns the next higher layer.
    #[must_use]
    pub const fn above(self) -> Self { Self(self.0 + 1) }
    /// Returns the next lower layer.
    #[must_use]
    pub const fn below(self) -> Self { Self(self.0 - 1) }
}
