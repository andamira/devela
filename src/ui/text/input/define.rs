// devela/src/ui/text/mod.rs
//
//! Defines `TextInput[Config|Action|Outcome|Reject|View]`.
//
// Boundary: TextInput owns text mutation and cursor state.
// It does not own focus, event acquisition, layout, or presentation.
//

#[doc = crate::_tags!(ui text)]
/// One-line editable text state over caller-chosen storage.
#[doc = crate::_doc_meta!{
    location("ui/text"),
    #[cfg(target_pointer_width = "32")]
    test_size_of(__: TextInput<&mut [u8]> = 28|224; niche Option),
    #[cfg(target_pointer_width = "64")]
    test_size_of(__: TextInput<&mut [u8]> = 56|448; niche Option),
}]
/// The storage type determines ownership:
/// - `TextInput<[u8; N]>` owns inline storage.
/// - `TextInput<&mut [u8]>` edits caller-provided storage.
/// - `TextInput<String>` uses growable allocated storage.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TextInput<B> {
    pub(crate) storage: B,
    pub(crate) len: usize,
    pub(crate) cursor: usize,
    pub(crate) config: TextInputConfig,
}

#[doc = crate::_tags!(ui text)]
/// Configuration for [`TextInput`].
#[doc = crate::_doc_meta!{
    location("ui/text"),
    #[cfg(target_pointer_width = "32")]
    test_size_of(TextInputConfig = 12|96; niche Option),
    #[cfg(target_pointer_width = "64")]
    test_size_of(TextInputConfig = 24|192; niche Option),
}]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TextInputConfig {
    /// Maximum initialized byte length.
    ///
    /// `None` means the storage capacity is the only limit.
    pub max_bytes: Option<usize>,

    /// Whether accepting an empty input is allowed.
    pub can_be_empty: bool,
}
impl TextInputConfig {
    /// Default text input configuration.
    pub const DEFAULT: Self = Self { max_bytes: None, can_be_empty: true };
}

#[doc = crate::_tags!(ui text interaction)]
/// Editing command consumed by [`TextInput`].
#[doc = crate::_doc_meta!{
    location("ui/text"),
    test_size_of(TextInputAction = 4|32; niche Option),
}]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TextInputAction {
    /// Inserts one Unicode scalar at the cursor.
    Insert(char),

    /// Deletes the character before the cursor.
    Backspace,

    /// Deletes the character at the cursor.
    Delete,

    /// Moves the cursor one character left.
    MoveLeft,

    /// Moves the cursor one character right.
    MoveRight,

    /// Moves the cursor to the beginning.
    MoveStart,

    /// Moves the cursor to the end.
    MoveEnd,

    /// Clears the input.
    Clear,

    /// Accepts the current input.
    Accept,

    /// Cancels the current input.
    Cancel,
}

#[doc = crate::_tags!(ui text result)]
/// Result of applying a [`TextInputAction`].
#[doc = crate::_doc_meta!{
    location("ui/text"),
    test_size_of(TextInputOutcome = 1|8; niche Option),
}]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TextInputOutcome {
    /// The action had no effect.
    Unchanged,

    /// The input text or cursor changed.
    Changed,

    /// The input was accepted.
    Accepted,

    /// The input was cancelled.
    Cancelled,

    /// The action was rejected.
    Rejected(TextInputReject),
}

#[doc = crate::_tags!(ui text)]
/// Reason why an action was rejected.
#[doc = crate::_doc_meta!{
    location("ui/text"),
    test_size_of(TextInputReject = 1|8; niche Option),
}]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TextInputReject {
    /// The storage or configured byte limit was reached.
    Full,

    /// Empty input was not accepted.
    Empty,

    /// The cursor was not at a valid UTF-8 boundary.
    InvalidCursor,
}

#[doc = crate::_tags!(ui text)]
/// Borrowed view of a text input.
#[doc = crate::_doc_meta!{
    location("ui/text"),
    #[cfg(target_pointer_width = "32")]
    test_size_of(TextInputView = 12|96; niche Option),
    #[cfg(target_pointer_width = "64")]
    test_size_of(TextInputView = 24|192; niche Option),
}]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TextInputView<'a> {
    /// The initialized UTF-8 text.
    pub text: &'a str,

    /// Cursor byte position inside `text`.
    pub cursor: usize,
}
