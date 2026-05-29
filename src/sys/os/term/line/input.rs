// devela::sys::os::term::line::input
//
//! Terminal line input modes.
//

#[doc = crate::_tags!(term interaction)]
/// How terminal input is delivered to an application.
#[doc = crate::_doc_meta!{location("sys/os/term")}]
///
/// This is a semantic line-discipline request. Backends apply it through their
/// native terminal state machinery, such as Linux termios.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TermLineMode {
    /// Line-buffered terminal input. This is the default.
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
