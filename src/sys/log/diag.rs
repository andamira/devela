// devela::sys::log::diag
//
//! Defines [`DiagLevel`], [`DiagOut`].
//
// FUTURE: DiagRecord

#[doc = crate::_tags!(log)]
/// The severity of a diagnostic emission.
#[doc = crate::_doc_location!("sys/log")]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Ord, PartialOrd)]
#[allow(missing_docs)]
pub enum DiagLevel {
    Trace,
    Debug,
    #[default]
    Info,
    Warn,
    Error,
}

#[doc = crate::_tags!(log)]
/// Emits leveled diagnostic text.
#[doc = crate::_doc_location!("sys/log")]
///
/// This is the minimal semantic sink for diagnostics.
/// It layers above plain text output by attaching a [`DiagLevel`]
/// to each emitted message.
///
/// See also [`TextOut`][crate::TextOut] for non-leveled textual output.
pub trait DiagOut {
    /// The error returned when diagnostic emission fails.
    type Error;

    /// Emits `text` with the given diagnostic `level`.
    fn diag(&mut self, level: DiagLevel, text: &str) -> Result<(), Self::Error>;

    /// Emits a trace diagnostic.
    fn trace(&mut self, text: &str) -> Result<(), Self::Error> {
        self.diag(DiagLevel::Trace, text)
    }
    /// Emits a debug diagnostic.
    fn debug(&mut self, text: &str) -> Result<(), Self::Error> {
        self.diag(DiagLevel::Debug, text)
    }
    /// Emits an informational diagnostic.
    fn info(&mut self, text: &str) -> Result<(), Self::Error> {
        self.diag(DiagLevel::Info, text)
    }
    /// Emits a warning diagnostic.
    fn warn(&mut self, text: &str) -> Result<(), Self::Error> {
        self.diag(DiagLevel::Warn, text)
    }
    /// Emits an error diagnostic.
    fn error(&mut self, text: &str) -> Result<(), Self::Error> {
        self.diag(DiagLevel::Error, text)
    }
}
