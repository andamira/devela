// devela/src/ui/text.rs
//
//! Defines [`UiText`].
//

use crate::UiId;

#[doc = crate::_tags!(ui)]
/// Human-readable text associated with a UI identity.
#[doc = crate::_doc_meta! {
    location("ui/semantic"),
    #[cfg(target_pointer_width = "32")]
    test_size_of(UiText = 24|192),
    #[cfg(target_pointer_width = "64")]
    test_size_of(UiText = 40|320),
}]
#[must_use]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct UiText<'a> {
    id: UiId,
    label: Option<&'a str>,
    description: Option<&'a str>,
}
#[rustfmt::skip]
impl<'a> UiText<'a> {
    /* constructors */

    /// Constructs an empty semantic text.
    pub const fn new(id: UiId) -> Self {
        Self::from_parts(id, None, None)
    }
    /// Constructs semantic text from all parts.
    pub const fn from_parts(id: UiId, label: Option<&'a str>, description: Option<&'a str>) -> Self {
        Self { id, label, description }
    }

    /* queries */
    
    /// Returns the UI identity.
    pub const fn id(&self) -> UiId { self.id }
    /// Returns the label.
    pub const fn label(&self) -> Option<&'a str> { self.label }
    /// Returns the description.
    pub const fn description(&self) -> Option<&'a str> { self.description }

    /// Returns whether no text is present.
    pub const fn is_empty(&self) -> bool {
        self.label.is_none() && self.description.is_none()
    }

    /* modifiers */

    /// Returns this text with a label.
    pub const fn with_label(self, label: &'a str) -> Self { Self { label: Some(label), ..self } }
    /// Returns this text without a label.
    pub const fn without_label(self) -> Self { Self { label: None, ..self } }

    /// Returns this text with a description.
    pub const fn with_description(self, description: &'a str) -> Self {
        Self { description: Some(description), ..self }
    }
    /// Returns this text without a description.
    pub const fn without_description(self) -> Self {
        Self { description: None, ..self }
    }

    /* setters */

    /// Sets the label.
    pub const fn set_label(&mut self, label: &'a str) { self.label = Some(label); }
    /// Clears the label.
    pub const fn clear_label(&mut self) { self.label = None; }

    /// Sets the description.
    pub const fn set_description(&mut self, description: &'a str) {
        self.description = Some(description);
    }
    /// Clears the description.
    pub const fn clear_description(&mut self) { self.description = None; }
}
