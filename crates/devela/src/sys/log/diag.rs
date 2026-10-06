//
//! Defines [`DiagLevel`], [`DiagOut`].
//
// FUTURE: DiagRecord

#[doc = crate::_tags!(log)]
/// The severity of a diagnostic emission.
#[doc = crate::_doc_meta!{
    location("sys/log", enum DiagLevel),
    test_size_of(DiagLevel = 1|8; niche Option),
}]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Ord, PartialOrd)]
#[allow(missing_docs)]
pub enum DiagLevel {
    Trace,
    Debug,
    #[doc = crate::_tags!(init)]
    #[default]
    Info,
    Warn,
    Error,
    Critical,
}

#[doc = crate::_tags!(log)]
/// Emits leveled diagnostic text.
#[doc = crate::_doc_meta!{
    location("sys/log", trait DiagOut),
}]
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
    /// Emits a critical diagnostic.
    fn critical(&mut self, text: &str) -> Result<(), Self::Error> {
        self.diag(DiagLevel::Critical, text)
    }
}

#[cfg(feature = "std")]
mod impl_std {
    use super::{DiagLevel, DiagOut};
    use crate::{IoError, IoWrite, Stderr};
    use std::io::StderrLock;

    fn emit(out: &mut impl IoWrite, level: DiagLevel, text: &str) -> Result<(), IoError> {
        let prefix: &[u8] = match level {
            DiagLevel::Trace => b"[trace] ",
            DiagLevel::Debug => b"[debug] ",
            DiagLevel::Info => b"[info] ",
            DiagLevel::Warn => b"[warn] ",
            DiagLevel::Error => b"[error] ",
            DiagLevel::Critical => b"[critical] ",
        };

        out.write_all(prefix)?;
        out.write_all(text.as_bytes())?;
        out.write_all(b"\n")
    }

    impl DiagOut for Stderr {
        type Error = IoError;

        fn diag(&mut self, level: DiagLevel, text: &str) -> Result<(), Self::Error> {
            emit(self, level, text)
        }
    }
    impl DiagOut for StderrLock<'_> {
        type Error = IoError;

        fn diag(&mut self, level: DiagLevel, text: &str) -> Result<(), Self::Error> {
            emit(self, level, text)
        }
    }
}
