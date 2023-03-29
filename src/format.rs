// devela::format
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

/// Returns a formatted string backed by a buffer, `no_std` compatible.
///
/// See also the [`format_buf!`][crate::format_buf!] macro.
///
/// # Examples
/// ```
/// use devela::format_buf;
///
/// let mut buf = [0u8; 64];
/// let s = format_buf(&mut buf, format_args!["Test: {} {}", "foo", 42]);
///
/// assert_eq!(Ok("Test: foo 42"), s);
/// ```
pub fn format_buf<'a>(buf: &'a mut [u8], arg: fmt::Arguments) -> Result<&'a str, fmt::Error> {
    let mut w = WriteTo::new(buf);
    fmt::write(&mut w, arg)?;
    w.as_str().ok_or(fmt::Error)
}

/// Returns a formatted string backed by a buffer, `no_std` compatible.
///
/// See also the [`format_buf`][format_buf()] function.
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
    ($buf:expr, $fmt:expr) => {
        $crate::format_buf($buf, format_args![$fmt])
    };

    ($buf:expr, $fmt:expr, $($args:tt)*) => {
        $crate::format_buf($buf, format_args![$fmt, $($args)*])
    };
}

#[derive(Debug)]
struct WriteTo<'a> {
    buf: &'a mut [u8],
    len: usize,
}

impl<'a> WriteTo<'a> {
    fn new(buf: &'a mut [u8]) -> Self {
        WriteTo { buf, len: 0 }
    }

    #[cfg(feature = "safe")]
    #[allow(clippy::wrong_self_convention)]
    fn as_str(self) -> Option<&'a str> {
        if self.len <= self.buf.len() {
            Some(from_utf8(&self.buf[..self.len]).expect("invalid utf-8"))
        } else {
            None
        }
    }

    #[cfg(not(feature = "safe"))]
    #[allow(clippy::wrong_self_convention)]
    fn as_str(self) -> Option<&'a str> {
        if self.len <= self.buf.len() {
            Some(unsafe { from_utf8_unchecked(&self.buf[..self.len]) })
        } else {
            None
        }
    }
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
