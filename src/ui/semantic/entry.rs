// devela/src/ui/entry.rs
//
//! Defines [`UiAction`].
//

use crate::{UiAction, UiActions, UiFlags, UiId, UiRole};

#[doc = crate::_tags!(ui)]
/// Compact semantic record for a UI identity.
#[doc = crate::_doc_meta! {
    location("ui/semantic"),
    test_size_of(UiEntry = 16|128),
}]
#[must_use]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct UiEntry {
    /// Resolved UI identity.
    id: UiId,

    /// Human-facing role.
    role: UiRole,

    /// Exposed semantic actions.
    actions: UiActions,

    /// Semantic state flags.
    flags: UiFlags,
}

#[rustfmt::skip]
impl UiEntry {
    /// Constructs a semantic entry.
    pub const fn new(id: UiId, role: UiRole) -> Self {
        Self { id, role, actions: UiActions::new(), flags: UiFlags::new() }
    }
    /// Constructs a semantic entry from all parts.
    pub const fn from_parts(id: UiId, role: UiRole, actions: UiActions, flags: UiFlags) -> Self {
        Self { id, role, actions, flags }
    }

    /// Returns the UI identity.
    #[must_use]
    pub const fn id(self) -> UiId { self.id }
    /// Returns the semantic role.
    #[must_use]
    pub const fn role(self) -> UiRole { self.role }
    /// Returns the exposed semantic actions.
    #[must_use]
    pub const fn actions(self) -> UiActions { self.actions }
    /// Returns the semantic state flags.
    #[must_use]
    pub const fn flags(self) -> UiFlags { self.flags }

    /// Returns this entry with updated actions.
    #[must_use]
    pub const fn with_actions(self, actions: UiActions) -> Self { Self { actions, ..self } }
    /// Returns this entry with updated flags.
    #[must_use]
    pub const fn with_flags(self, flags: UiFlags) -> Self { Self { flags, ..self } }

    /// Returns this entry with one action included.
    #[must_use]
    pub const fn with_action(self, action: UiAction) -> Self {
        Self { actions: self.actions.with(action.to_set()), ..self }
    }
}
