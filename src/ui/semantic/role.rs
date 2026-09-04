// devela/src/ui/role.rs
//
//! Defines [`UiRole`].
//

#[doc = crate::_tags!(ui)]
/// Human-facing role of a UI identity.
#[doc = crate::_doc_meta! {
    location("ui/semantic", enum UiRole),
    test_size_of(UiRole = 1|8; niche Option),
}]
#[must_use]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum UiRole {
    #[doc = crate::_tags!(init)]
    /// No semantic role.
    #[default]
    None,

    /// Non-interactive text or symbolic content.
    Label,

    /// Activatable command control.
    Button,

    /// Boolean toggle control.
    Checkbox,

    /// Mutually exclusive choice control.
    Radio,

    /// Selectable item in a collection.
    Option,

    /// Editable text input.
    TextInput,

    /// Numeric or continuous value control.
    Slider,

    /// Progress indicator.
    Progress,

    /// Grouping container.
    Group,

    /// Image or non-text visual content.
    Image,

    /// Link to another location or resource.
    Link,
}
