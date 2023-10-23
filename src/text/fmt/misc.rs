// devela::text::fmt::misc
//
//! Miscellaneous formatting functionality.
//
// DOCS
// - https://doc.rust-lang.org/core/fmt/trait.Write.html
// - https://doc.rust-lang.org/nightly/core/macro.format_args.html
// - https://doc.rust-lang.org/nightly/core/fmt/struct.Arguments.html
// - https://stackoverflow.com/questions/50200268/how-can-i-use-the-format-macro-in-a-no-std-environment

#[allow(unused_imports)]
use core::{
    cmp::min,
    fmt,
    str::{from_utf8, from_utf8_unchecked},
};

#[cfg(feature = "alloc")]
use _alloc::{format, string::String};

/// A more *`c`ompact [`dbg!`]*. Uses `{:?}` instead of `{:#?}` for formatting.
///
/// # Examples
/// ```
/// use devela::text::cdbg;
///
/// let a = vec![1, 2, 3];
/// let _b = cdbg![a];
/// //       ^-- prints: [src/main.rs:5] a = [1, 2, 3]
/// ```
// Source code based on the original `dbg!` implementation.
#[macro_export]
macro_rules! cdbg {
    () => {
        eprintln!("[{}:{}]", file!(), line!())
    };
    ($val:expr $(,)?) => {
        match $val {
            tmp => {
                eprintln!("[{}:{}] {} = {:?}", // <- KEY CHANGE
                    file!(), line!(), stringify!($val), &tmp);
                tmp
            }
        }
    };
    ($($val:expr),+ $(,)?) => {
        ($(cdbg!($val)),+,)
    };
}
pub use cdbg;

/// Returns a formatted [`str`] slice backed by a buffer, `no_std` compatible.
///
/// See also the slightly more convenient to use [`format_buf!`][crate::format_buf!] macro.
///
/// # Examples
/// ```
/// use devela::text::format_buf_args;
///
/// let mut buf = [0u8; 64];
/// let s = format_buf_args(&mut buf, format_args!["Test: {} {}", "foo", 42]);
///
/// assert_eq!(Ok("Test: foo 42"), s);
/// ```
pub fn format_buf_args<'a>(buf: &'a mut [u8], arg: fmt::Arguments) -> Result<&'a str, fmt::Error> {
    let mut w = WriteTo::new(buf);
    fmt::write(&mut w, arg)?;
    w.as_str().ok_or(fmt::Error)
}

/// Returns a formatted [`str`] slice backed by a buffer, `no_std` compatible.
///
/// It calls the [`format_buf_args`][format_buf_args()] function with the
/// [`format_args`] macro.
///
/// # Examples
/// ```
/// use devela::format_buf;
///
/// let mut buf = [0u8; 64];
/// let s = format_buf![&mut buf, "Test: {} {}", "foo", 42];
///
/// assert_eq!(Ok("Test: foo 42"), s);
/// ```
#[macro_export]
macro_rules! format_buf {
    ($buf:expr, $($args:tt)*) => {
        $crate::text::fmt::format_buf_args($buf, format_args![$($args)*])
    };
}
pub use format_buf;

#[derive(Debug)]
struct WriteTo<'a> {
    buf: &'a mut [u8],
    len: usize,
}

impl<'a> fmt::Write for WriteTo<'a> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        if self.len > self.buf.len() {
            return Err(fmt::Error);
        }

        let rem = &mut self.buf[self.len..];
        let raw_s = s.as_bytes();
        let num = min(raw_s.len(), rem.len());

        rem[..num].copy_from_slice(&raw_s[..num]);
        self.len += raw_s.len();

        if num < raw_s.len() {
            Err(fmt::Error)
        } else {
            Ok(())
        }
    }
}

impl<'a> WriteTo<'a> {
    fn new(buf: &'a mut [u8]) -> Self {
        WriteTo { buf, len: 0 }
    }

    #[cfg(not(feature = "unsafe_text"))]
    fn as_str(self) -> Option<&'a str> {
        if self.len <= self.buf.len() {
            Some(from_utf8(&self.buf[..self.len]).expect("invalid utf-8"))
        } else {
            None
        }
    }
    #[cfg(feature = "unsafe_text")]
    fn as_str(self) -> Option<&'a str> {
        if self.len <= self.buf.len() {
            // SAFETY: the buffer is always filled from a previous &str
            Some(unsafe { from_utf8_unchecked(&self.buf[..self.len]) })
        } else {
            None
        }
    }
}

/// An alternative `Debug`.
///
/// # Examples
///
/// Basic usage:
/// ```
/// use devela::text::AltDebug;
///
/// #[derive(Debug)]
/// struct Example(bool, i32, String);
///
/// impl AltDebug for Example {}
///
/// let example = Example(true, 4, "2+2".into());
/// assert_eq!["Example(true, 4, \"2+2\")", &example.to_dbg()];
/// ```
///
/// Custom usage:
/// ```
/// use devela::text::AltDebug;
///
/// #[derive(Debug)]
/// struct Example(bool, i32, String);
///
/// impl AltDebug for Example {
///     fn to_dbg(&self) -> String {
///         format!["{2}={1}:{0}", self.0, self.1, self.2]
///     }
/// }
///
/// let example = Example(true, 4, "2+2".into());
/// assert_eq!["2+2=4:true", &example.to_dbg()];
/// ```
// IMPROVE: use alternative formatting.
#[cfg(feature = "alloc")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "alloc")))]
pub trait AltDebug: core::fmt::Debug {
    /// Converts the current instance into a debug string.
    ///
    /// By default, this method leverages the standard `Debug` trait to create
    /// the debug representation.
    ///
    /// Implementers can override this method to provide a custom debug string.
    fn to_dbg(&self) -> String {
        format!["{self:?}"]
    }
}
