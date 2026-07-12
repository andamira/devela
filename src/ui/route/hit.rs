// devela/ui/route/hit.rs
//
//! Defines [`HitRegion`].
//
// Boundary: layout defines the geometry vocabulary;
// routing only associates a layout region with a resolved interaction identity.
//

use crate::{UiId, UiRect};

#[doc = crate::_tags!(ui)]
/// Frame-local region that can receive routed interaction.
#[doc = crate::_doc_meta! {
    location("ui/route"),
    test_size_of(HitRegion = 24|192),
}]
///
/// Connects a resolved UI identity with the layout region used for hit testing
/// and interaction routing.
#[must_use]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct HitRegion {
    /// Resolved UI identity.
    id: UiId,

    /// Interactive layout region.
    rect: UiRect,
}

impl HitRegion {
    /// Constructs a hit region from an identity and layout region.
    pub const fn new(id: UiId, rect: UiRect) -> Self {
        Self { id, rect }
    }
    /// Returns the UI identity.
    pub const fn id(self) -> UiId {
        self.id
    }
    /// Returns the interactive layout region.
    pub const fn rect(self) -> UiRect {
        self.rect
    }
}
