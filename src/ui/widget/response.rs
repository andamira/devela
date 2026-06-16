// devela/src/ui/widget/response.rs
//
//! Defines [`UiResponseFlags`], [`UiResponse`].
//

use crate::{_impl_init, UiId};

crate::set! {
    #[doc = crate::_tags!(ui set)]
    /// Interaction response flags produced by a UI item.
    #[doc = crate::_doc_meta! {
        location("ui/widget"),
        test_size_of(UiResponseFlags = 1|8),
    }]
    #[must_use]
    #[repr(transparent)]
    pub struct UiResponseFlags(u8) {
        /// Current pointer/cursor candidate.
        HOT = 0;

        /// Currently pressed, dragged, or otherwise engaged.
        ACTIVE = 1;

        /// Current keyboard/navigation focus owner.
        FOCUSED = 2;

        /// Activated during this frame.
        ACTIVATED = 3;

        /// Value or state changed during this frame.
        CHANGED = 4;
    }
}

#[doc = crate::_tags!(ui)]
/// Interaction result for a UI item.
#[doc = crate::_doc_meta! {
    location("ui/widget"),
    #[cfg(target_pointer_width = "32")]
    test_size_of(UiResponse = 12|96),
    #[cfg(target_pointer_width = "64")]
    test_size_of(UiResponse = 16|128),
}]
/// Captures what happened to one UI identity during the current frame,
/// such as focus, activation, or value changes.
#[must_use]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct UiResponse {
    id: UiId,
    flags: UiResponseFlags,
}
_impl_init![UiResponse::new(UiId::ROOT) => UiResponse];

#[rustfmt::skip]
impl UiResponse {
    /* constructors */

    /// Constructs an empty response for an identity.
    pub const fn new(id: UiId) -> Self {
        Self { id, flags: UiResponseFlags::new() }
    }
    /// Constructs a response from all parts.
    pub const fn from_parts(id: UiId, flags: UiResponseFlags) -> Self {
        Self { id, flags }
    }

    /* queries */

    /// Returns the UI identity.
    #[must_use]
    pub const fn id(self) -> UiId { self.id }

    /// Returns the response flags.
    #[must_use]
    pub const fn flags(self) -> UiResponseFlags { self.flags }

    /// Returns whether this response has no flags.
    #[must_use]
    pub const fn is_empty(self) -> bool { self.flags.is_empty() }
    /// Returns whether the item is hot.
    #[must_use]
    pub const fn is_hot(self) -> bool { self.flags.has(UiResponseFlags::HOT) }
    /// Returns whether the item is active.
    #[must_use]
    pub const fn is_active(self) -> bool { self.flags.has(UiResponseFlags::ACTIVE) }
    /// Returns whether the item is focused.
    #[must_use]
    pub const fn is_focused(self) -> bool { self.flags.has(UiResponseFlags::FOCUSED) }

    /// Returns whether the item was activated this frame.
    #[must_use]
    pub const fn is_activated(self) -> bool { self.flags.has(UiResponseFlags::ACTIVATED) }
    /// Returns whether the item changed this frame.
    #[must_use]
    pub const fn is_changed(self) -> bool { self.flags.has(UiResponseFlags::CHANGED) }

    /* modifiers */


    /// Returns this response with another flag set.
    #[must_use]
    pub const fn replace_flags(self, flags: UiResponseFlags) -> Self {
        Self { flags, ..self }
    }
    /// Returns this response with `flags` included.
    #[must_use]
    pub const fn with_flags(self, flags: UiResponseFlags) -> Self {
        Self { flags: self.flags.with(flags), ..self }
    }
    /// Returns this response with `flags` removed.
    #[must_use]
    pub const fn without_flags(self, flags: UiResponseFlags) -> Self {
        Self { flags: self.flags.without(flags), ..self }
    }

    /// Returns this response marked as hot.
    #[must_use]
    pub const fn hot(self) -> Self { self.with_flags(UiResponseFlags::HOT) }
    /// Returns this response marked as active.
    #[must_use]
    pub const fn active(self) -> Self { self.with_flags(UiResponseFlags::ACTIVE) }
    /// Returns this response marked as focused.
    #[must_use]
    pub const fn focused(self) -> Self { self.with_flags(UiResponseFlags::FOCUSED) }

    /// Returns this response marked as activated.
    #[must_use]
    pub const fn activate(self) -> Self { self.with_flags(UiResponseFlags::ACTIVATED) }
    /// Returns this response marked as changed.
    #[must_use]
    pub const fn change(self) -> Self { self.with_flags(UiResponseFlags::CHANGED) }
}
