// devela/src/ui/widget/button.rs
//
//! Defines [`UiButton`].
//

use crate::{UiAction, UiEntry, UiKey, UiRole, UiText};

#[doc = crate::_tags!(ui)]
/// Activatable command control.
#[doc = crate::_doc_meta! {
    location("ui/widget"),
    #[cfg(target_pointer_width = "32")]
    test_size_of(UiButton = 24|192),
    #[cfg(target_pointer_width = "64")]
    test_size_of(UiButton = 40|320),
}]
/// Carries the stable key and human text
/// used to describe a button across UI frames.
#[must_use]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct UiButton<'a> {
    key: UiKey,
    label: &'a str,
    description: Option<&'a str>,
}

#[rustfmt::skip]
impl<'a> UiButton<'a> {
    /* constructors */

    /// Constructs a button from a key and label.
    pub const fn new(key: UiKey, label: &'a str) -> Self {
        Self { key, label, description: None }
    }
    /// Constructs a button from all parts.
    pub const fn from_parts(key: UiKey, label: &'a str, description: Option<&'a str>) -> Self {
        Self { key, label, description }
    }

    /* queries */

    /// Returns the stable identity seed.
    #[must_use]
    pub const fn key(&self) -> UiKey { self.key }
    /// Returns the button label.
    #[must_use]
    pub const fn label(&self) -> &'a str { self.label }
    /// Returns the button description.
    #[must_use]
    pub const fn description(&self) -> Option<&'a str> { self.description }

    /* modifiers */

    /// Returns this button with another key.
    #[must_use]
    pub const fn with_key(self, key: UiKey) -> Self {
        Self { key, ..self }
    }
    /// Returns this button with another label.
    #[must_use]
    pub const fn with_label(self, label: &'a str) -> Self {
        Self { label, ..self }
    }
    /// Returns this button with a description.
    #[must_use]
    pub const fn with_description(self, description: &'a str) -> Self {
        Self { description: Some(description), ..self }
    }
    /// Returns this button without a description.
    #[must_use]
    pub const fn without_description(self) -> Self {
        Self { description: None, ..self }
    }

    /* semantic helpers */

    /// Returns the semantic entry for the resolved identity.
    #[must_use]
    pub const fn entry(&self, id: crate::UiId) -> UiEntry {
        UiEntry::new(id, UiRole::Button).with_action(UiAction::Activate)
    }
    /// Returns the semantic text for the resolved identity.
    #[must_use]
    pub const fn text(&self, id: crate::UiId) -> UiText<'a> {
        UiText::from_parts(id, Some(self.label), self.description)
    }
}
