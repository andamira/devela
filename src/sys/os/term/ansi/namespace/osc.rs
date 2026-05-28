// devela::sys::os::term::ansi::namespace::osc
//
//! Implements OSC sequences and defines [`AnsiLink`], [`AnsiOsc`].
//
// TOC
// - impl Ansi
// - struct AnsiOsc
// - struct AnsiLink
// - helpers
//   - _put_bytes
//   - _put!

use crate::{__ansi_consts, Ansi, Display, FmtResult, Formatter, is, whilst};

/// # Operating system commands
///
/// OSC sequences are terminal commands introduced by `ESC ]`.
impl Ansi {
    /// Returns an OSC sequence that sets the icon name and window title. (OSC 0)
    pub const fn title_all(text: &str) -> AnsiOscText<'_> {
        AnsiOsc::text("0", text)
    }
    /// Returns an OSC sequence that sets the icon name. (OSC 1)
    pub const fn title_icon(text: &str) -> AnsiOscText<'_> {
        AnsiOsc::text("1", text)
    }
    /// Returns an OSC sequence that sets the window title. (OSC 2)
    pub const fn title_window(text: &str) -> AnsiOscText<'_> {
        AnsiOsc::text("2", text)
    }

    /// Returns a terminal hyperlink wrapper.
    ///
    /// The returned value formats `text` as an OSC 8 hyperlink pointing to `uri`.
    ///
    /// # Notes
    /// The text and URI are emitted as-is. Use trusted or pre-sanitized input.
    // https://github.com/Alhadis/OSC8-Adoption/
    pub const fn link<'a>(text: &'a str, uri: &'a str) -> AnsiLink<'a> {
        AnsiLink::new(text, uri)
    }
    /// Returns a terminal hyperlink wrapper with an explicit hyperlink id.
    ///
    /// Terminals may use the id to connect separated hyperlink spans
    /// that point to the same target.
    pub const fn link_with_id<'a>(text: &'a str, uri: &'a str, id: &'a str) -> AnsiLink<'a> {
        AnsiLink::with_id(text, uri, id)
    }
    __ansi_consts! {
        /// Closes the current OSC 8 hyperlink span.
        ///
        /// This is emitted automatically by [`AnsiLink`]'s implementation.
        pub const HYPERLINK_CLOSE: [u8;7] = "\x1b]8;;\x1b\\", *b"\x1b]8;;\x1b\\";
    }

    /// Returns an OSC 52 sequence that sets selection data from base64 text.
    pub const fn clipboard_base64<'a>(selection: &'a str, base64: &'a str) -> AnsiOsc<'a, 2> {
        AnsiOsc::new("52", [selection, base64])
    }
    /// Returns an OSC 52 sequence that requests the clipboard contents.
    ///
    /// The terminal may answer with an OSC 52 response on the input stream.
    pub const fn clipboard_query_clipboard() -> AnsiOsc<'static, 2> {
        AnsiOsc::new("52", ["c", "?"])
    }
    /// Returns an OSC 52 sequence that requests selection data.
    pub const fn clipboard_query<'a>(selection: &'a str) -> AnsiOsc<'a, 2> {
        AnsiOsc::new("52", [selection, "?"])
    }
}

#[doc = crate::_tags!(term string)]
/// An OSC sequence with semicolon-separated fields.
#[doc = crate::_doc_meta!{
    location("sys/os/term"),
    #[cfg(target_pointer_width = "32")]
    test_size_of(AnsiOsc1: AnsiOsc<'_, 1> = 16|128),
    #[cfg(target_pointer_width = "64")]
    test_size_of(AnsiOsc1: AnsiOsc<'_, 1> = 32|256),
}]
#[must_use]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct AnsiOsc<'a, const N: usize> {
    /// The OSC command number, encoded as ASCII digits.
    pub command: &'static str,
    /// The semicolon-separated OSC fields.
    pub fields: [&'a str; N],
}
impl<'a, const N: usize> AnsiOsc<'a, N> {
    /// Creates a new OSC sequence.
    pub const fn new(command: &'static str, fields: [&'a str; N]) -> Self {
        Self { command, fields }
    }
    /// Returns the number of bytes needed to encode this OSC sequence.
    #[must_use]
    pub const fn len(self) -> usize {
        let mut len = 2 + self.command.len() + 2; // ESC ] + command + ST
        whilst! { i in 0..N; {
            len += 1 + self.fields[i].len(); // ;field
        }}
        len
    }
    /// Returns whether the encoded OSC sequence is empty.
    #[must_use]
    pub const fn is_empty(self) -> bool {
        self.len() == 0
    }
    /// Writes this OSC sequence into `dst`, returning the final offset.
    ///
    /// Returns `Err(needed)` if the slice is too small.
    pub const fn write_to(self, dst: &mut [u8]) -> Result<usize, usize> {
        let needed = self.len();
        let mut offset = 0;
        _put!(dst, offset, needed, b"\x1b]");
        _put!(dst, offset, needed, self.command.as_bytes());
        whilst! { i in 0..N; {
            _put!(dst, offset, needed, b";");
            _put!(dst, offset, needed, self.fields[i].as_bytes());
        }}
        _put!(dst, offset, needed, b"\x1b\\");
        Ok(offset)
    }
    fn fmt_streamed(self, f: &mut Formatter<'_>) -> FmtResult<()> {
        f.write_str("\x1b]")?;
        f.write_str(self.command)?;
        whilst! { i in 0..N; {
            f.write_str(";")?;
            f.write_str(self.fields[i])?;
        }}
        f.write_str("\x1b\\")
    }
}
impl<const N: usize> Display for AnsiOsc<'_, N> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult<()> {
        self.fmt_streamed(f)
    }
}

/// An OSC sequence carrying one text field.
type AnsiOscText<'a> = AnsiOsc<'a, 1>;
impl<'a> AnsiOsc<'a, 1> {
    /// Creates an OSC sequence carrying one text field.
    pub const fn text(command: &'static str, text: &'a str) -> Self {
        Self::new(command, [text])
    }
}

#[doc = crate::_tags!(term web string)]
/// A terminal hyperlink using the OSC 8 escape sequence.
#[doc = crate::_doc_meta!{
    location("sys/os/term"),
    #[cfg(target_pointer_width = "32")]
    test_size_of(AnsiLink: AnsiLink<'_> = 24|192),
    #[cfg(target_pointer_width = "64")]
    test_size_of(AnsiLink: AnsiLink<'_> = 48|384),
}]
///
/// Formatting this value emits:
///
/// ```text
/// OSC 8 ; params ; URI ST
/// text
/// OSC 8 ; ; ST
/// ```
///
/// # Notes
/// Support depends on the terminal emulator. Unsupported terminals usually
/// display only the visible text or ignore the sequence.
#[must_use]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct AnsiLink<'a> {
    /// The visible terminal text.
    pub text: &'a str,
    /// The hyperlink target URI.
    pub uri: &'a str,
    /// Optional OSC 8 hyperlink id.
    ///
    /// Terminals may use this to associate separated spans with the same link.
    pub id: Option<&'a str>,
}
impl<'a> AnsiLink<'a> {
    /// Creates a new terminal hyperlink.
    pub const fn new(text: &'a str, uri: &'a str) -> Self {
        Self { text, uri, id: None }
    }
    /// Creates a new terminal hyperlink with an explicit id.
    pub const fn with_id(text: &'a str, uri: &'a str, id: &'a str) -> Self {
        Self { text, uri, id: Some(id) }
    }
    /// Returns the number of bytes needed to encode this hyperlink.
    #[must_use]
    pub const fn len(self) -> usize {
        let id_len = match self.id {
            Some(id) => 3 + id.len(), // "id=" + id
            None => 0,
        };
        // ESC ] 8 ; params ; uri ST + text + close
        4 + id_len + 1 + self.uri.len() + 2 + self.text.len() + Ansi::HYPERLINK_CLOSE_B.len()
    }
    /// Returns whether this hyperlink encodes to no bytes.
    ///
    /// This is always `false` for the current OSC 8 encoding, but is useful
    /// for API symmetry with byte-oriented values.
    #[must_use]
    pub const fn is_empty(self) -> bool {
        self.len() == 0
    }
    /// Writes this hyperlink into `dst`, returning the final offset.
    ///
    /// Returns `Err(needed)` if the slice is too small.
    ///
    /// The text, URI and id are emitted as-is.
    pub const fn write_to(self, dst: &mut [u8]) -> Result<usize, usize> {
        let needed = self.len();
        let mut offset = 0;
        _put!(dst, offset, needed, b"\x1b]8;");
        if let Some(id) = self.id {
            _put!(dst, offset, needed, b"id=");
            _put!(dst, offset, needed, id.as_bytes());
        }
        _put!(dst, offset, needed, b";");
        _put!(dst, offset, needed, self.uri.as_bytes());
        _put!(dst, offset, needed, b"\x1b\\");
        _put!(dst, offset, needed, self.text.as_bytes());
        _put!(dst, offset, needed, &Ansi::HYPERLINK_CLOSE_B);
        Ok(offset)
    }
    fn fmt_streamed(self, f: &mut Formatter<'_>) -> FmtResult<()> {
        f.write_str("\x1b]8;")?;
        if let Some(id) = self.id {
            f.write_str("id=")?;
            f.write_str(id)?;
        }
        f.write_str(";")?;
        f.write_str(self.uri)?;
        f.write_str("\x1b\\")?;
        f.write_str(self.text)?;
        f.write_str(Ansi::HYPERLINK_CLOSE)
    }
}
#[rustfmt::skip]
impl Display for AnsiLink<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult<()> { self.fmt_streamed(f) }
}

/* helpers */

#[inline(always)] #[rustfmt::skip]
const fn _put_bytes(dst: &mut [u8], offset: usize, src: &[u8], needed: usize)
    -> Result<usize, usize> {
    is! { offset > dst.len() || src.len() > dst.len() - offset, return Err(needed) }
    whilst! { i in 0..src.len(); { dst[offset + i] = src[i]; }}
    Ok(offset + src.len())
}
macro_rules! _put {
    ($dst:expr, $offset:ident, $needed:expr, $src:expr) => {
        match _put_bytes($dst, $offset, $src, $needed) {
            Ok(new_offset) => $offset = new_offset,
            Err(needed) => return Err(needed),
        }
    };
}
use _put;
