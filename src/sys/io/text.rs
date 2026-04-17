// devela::sys::io::text
//
//! Defines [`TextIn`], [`TextOut`].
//
// TOC
// - trait TextIn
// - trait TextOut
// - impls TextOut

#[cfg(doc)]
use crate::IoRead;
use crate::{Fmt, FmtArguments, FmtError, FmtResult, FmtWrite};
#[cfg(doc)]
use crate::{IoError, IoWrite};

#[doc = crate::_tags!(io text)]
/// Reads UTF-8 text from an input surface into caller-provided storage.
#[doc = crate::_doc_location!("sys/io")]
///
/// This is the text-layer counterpart to [`IoRead`].
/// It is intended for sources whose primary semantic payload is text,
/// while keeping storage explicit and allocation optional.
///
/// See also [`IoRead`] for byte-oriented input.
pub trait TextIn {
    /// The error returned when text input fails.
    type Error;

    /// Reads some UTF-8 text into `buf`, returning the written subslice.
    fn read_text<'a>(&mut self, buf: &'a mut [u8]) -> Result<&'a str, Self::Error>;
}

#[doc = crate::_tags!(io text)]
/// Emits UTF-8 text to an output surface.
#[doc = crate::_doc_location!("sys/io")]
///
/// This is the text-layer counterpart to [`IoWrite`].
/// It is intended for sinks whose primary semantic payload is text,
/// such as terminals, log files, browser text surfaces, or formatted buffers.
///
/// See also [`IoWrite`] for byte-oriented output.
pub trait TextOut {
    /* required */

    /// The error returned when text emission fails.
    type Error;

    /// Emits `text` without appending a line ending.
    fn write_text(&mut self, text: &str) -> Result<(), Self::Error>;

    /* provided */

    /// Emits `text` followed by `\n`.
    fn write_line(&mut self, text: &str) -> Result<(), Self::Error> {
        self.write_text(text)?;
        self.write_text("\n")
    }

    /// Emits a single Unicode scalar.
    fn write_char(&mut self, c: char) -> Result<(), Self::Error> {
        let mut buf = [0u8; 4];
        self.write_text(c.encode_utf8(&mut buf))
    }

    /// Emits formatted text.
    fn write_fmt(&mut self, args: FmtArguments<'_>) -> Result<(), Self::Error> {
        struct Adapter<'a, T: ?Sized + TextOut> {
            out: &'a mut T,
            err: Option<T::Error>,
        }

        impl<T: ?Sized + TextOut> FmtWrite for Adapter<'_, T> {
            fn write_str(&mut self, s: &str) -> FmtResult<()> {
                match self.out.write_text(s) {
                    Ok(()) => Ok(()),
                    Err(e) => {
                        self.err = Some(e);
                        Err(FmtError)
                    }
                }
            }
        }
        let mut a = Adapter { out: self, err: None };
        match Fmt::write(&mut a, args) {
            Ok(()) => Ok(()),
            Err(_) => Err(a.err.expect("TextOut::write_fmt adapter lost inner error")),
        }
    }
}

/*
/// Adapts any byte-oriented sink into a UTF-8 text sink.
///
/// This blanket implementation forwards each `&str` as UTF-8 bytes through
/// [`IoWrite`], without adding buffering, styling, or line-ending conversion.
///
/// It is convenient, but also broad: once published, downstream crates cannot
/// provide a more specific [`TextOut`] implementation for a type that already
/// implements [`IoWrite`].
impl<T: IoWrite + ?Sized> TextOut for T {
    type Error = IoError;
    fn write_text(&mut self, text: &str) -> Result<(), Self::Error> {
        self.write_all(text.as_bytes())
    }
}
*/

#[cfg(feature = "io")]
/// Discards all emitted text.
impl TextOut for crate::IoEmpty {
    type Error = crate::IoError;
    fn write_text(&mut self, text: &str) -> Result<(), Self::Error> {
        let _ = text;
        Ok(())
    }
}
/// Appends emitted text into an owned UTF-8 buffer.
///
/// This implementation is infallible and useful for capture, accumulation,
/// deferred presentation, and text-backed diagnostic sinks.
#[cfg(feature = "alloc")]
impl TextOut for crate::String {
    type Error = crate::Infallible;
    fn write_text(&mut self, text: &str) -> Result<(), Self::Error> {
        self.push_str(text);
        Ok(())
    }
}
#[cfg(feature = "std")]
mod impl_std {
    use crate::{IoError, IoWrite, TextOut};
    use crate::{Stderr, StderrLock, Stdout, StdoutLock};

    /// Emits UTF-8 text to the process standard output stream.
    impl TextOut for Stdout {
        type Error = IoError;
        fn write_text(&mut self, text: &str) -> Result<(), Self::Error> {
            self.write_all(text.as_bytes())
        }
    }
    /// Emits UTF-8 text to a locked standard output stream.
    impl TextOut for StdoutLock<'_> {
        type Error = IoError;
        fn write_text(&mut self, text: &str) -> Result<(), Self::Error> {
            self.write_all(text.as_bytes())
        }
    }
    /// Emits UTF-8 text to the process standard error stream.
    impl TextOut for Stderr {
        type Error = IoError;
        fn write_text(&mut self, text: &str) -> Result<(), Self::Error> {
            self.write_all(text.as_bytes())
        }
    }
    /// Emits UTF-8 text to a locked standard error stream.
    impl TextOut for StderrLock<'_> {
        type Error = IoError;
        fn write_text(&mut self, text: &str) -> Result<(), Self::Error> {
            self.write_all(text.as_bytes())
        }
    }
}
