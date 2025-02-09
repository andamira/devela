// devela::text::fmt::namespace
//
//! [`Fmt`] namespace.
//

use super::WriteTo;
use crate::{FmtArguments, FmtError, FmtWrite, _core::fmt::write};
#[cfg(feature = "alloc")]
use crate::{String, _dep::_alloc::fmt::format};

#[doc = crate::TAG_NAMESPACE!()]
/// A string formatting namespace.
///
/// See also the [`std::fmt`] module.
pub struct Fmt;

///
impl Fmt {
    /// Takes an [`FmtArguments`] struct and returns the resulting formatted string.
    ///
    /// The `FmtArguments` instance can be created with the [`format_args!`] macro.
    ///
    /// See `alloc::fmt::`[`format`][fn@format].
    ///
    /// [`format_args!`]: crate::format_args
    #[must_use]
    #[cfg(feature = "alloc")]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "alloc")))]
    pub fn format(args: FmtArguments<'_>) -> String {
        format(args)
    }

    /// Writes formatted output into the given byte buffer.
    ///
    /// Returns:
    /// - `Ok(&str)` if all the formatted data fits into `buf`.
    /// - `Err(&str)` containing the valid partial result if truncation occurred.
    ///
    /// # Example
    /// ```
    /// # use devela::Fmt;
    /// let mut buf = [0u8; 32]; // Big enough to fit everything
    /// let s = Fmt::format_buf(&mut buf, format_args!["Test: {} {}", "foo", 42]);
    /// assert_eq!(Ok("Test: foo 42"), s);
    ///
    /// let mut buf = [0u8; 9]; // Can't fit everything
    /// let s = Fmt::format_buf(&mut buf, format_args!["Test: {} {}", "foo", 42]);
    /// assert_eq!(Err("Test: foo"), s);
    /// ```
    pub fn format_buf<'a>(buf: &'a mut [u8], args: FmtArguments) -> Result<&'a str, &'a str> {
        let mut w = WriteTo::new(buf);
        let _ = Fmt::write(&mut w, args);
        if w.truncated {
            Err(w.as_str())
        } else {
            Ok(w.as_str())
        }
    }

    /// Takes an output stream and an `FmtArguments` struct
    /// that can be precompiled with the [`format_args!`] macro.
    ///
    /// The arguments will be formatted according to the specified format string
    /// into the output stream provided.
    ///
    /// See `core::fmt::`[`write`][fn@write].
    ///
    /// [`format_args!`]: crate::format_args
    pub fn write(output: &mut dyn FmtWrite, args: FmtArguments<'_>) -> Result<(), FmtError> {
        write(output, args)
    }

    // /// Creates a type whose `fmt::Debug` and `fmt::Display` impls
    // /// are provided with the function `f`.
    // // WAIT: [debug_closure_helpers](https://github.com/rust-lang/rust/issues/117729)
    // pub fn from_fn<F>(f: F) -> FromFn<F>
    // where
    //     F: Fn(&mut Formatter<'_>) -> Result<(), Error>,
    // {
    //     crate::_core::fmt::from_fn(f)
    // }
}
