// devela/src/sys/os/term/line/input.rs
//
//! Terminal line input modes.
//

#[doc = crate::_tags!(term interaction)]
/// How terminal input is delivered to an application.
#[doc = crate::_doc_meta!{
    location("sys/os/term", enum TermLineMode),
    test_size_of(TermLineMode = 1|8; niche Option),
}]
/// This is a semantic line-discipline request. Backends apply it through their
/// native terminal state machinery, such as Linux termios.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TermLineMode {
    #[doc = crate::_tags!(init)]
    /// Line-buffered terminal input.
    #[default]
    Line,

    /// Event-oriented input with normal terminal behavior mostly preserved.
    Event,

    /// Raw byte-oriented input with most terminal processing disabled.
    Raw,
}

#[allow(non_upper_case_globals)]
impl TermLineMode {
    /// Traditional name for [`Line`](Self::Line).
    pub const Cooked: Self = Self::Line;

    /// Traditional name for [`Event`](Self::Event).
    pub const Cbreak: Self = Self::Event;
}
