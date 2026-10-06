//
//! Defines [`AndroidLog`].
//

use crate::{CStr, macro_apply};
#[macro_apply(crate::_android)]
use {
    super::_raw,
    crate::{DiagLevel, DiagOut, InvalidText, MismatchedCapacity, c_char, c_int, is},
};

#[doc = crate::_tags!(platform log)]
/// Android native logger with a fixed log tag.
#[doc = crate::_doc_meta!{
    location("sys/os/android", struct AndroidLog),
    #[cfg(target_pointer_width = "32")]
    test_size_of(AndroidLog = 8|64; niche Option),
    #[cfg(target_pointer_width = "64")]
    test_size_of(AndroidLog = 16|128; niche Option),
}]
#[derive(Clone, Copy, Debug)]
pub struct AndroidLog<'a> {
    tag: &'a CStr,
}

impl<'a> AndroidLog<'a> {
    /// Creates a logger using `tag`.
    #[must_use]
    pub const fn new(tag: &'a CStr) -> Self {
        Self { tag }
    }

    /// Returns the logger tag.
    #[must_use]
    pub const fn tag(&self) -> &'a CStr {
        self.tag
    }

    #[macro_apply(crate::_android)]
    const fn priority(level: DiagLevel) -> c_int {
        match level {
            DiagLevel::Trace => _raw::ANDROID_LOG_VERBOSE,
            DiagLevel::Debug => _raw::ANDROID_LOG_DEBUG,
            DiagLevel::Info => _raw::ANDROID_LOG_INFO,
            DiagLevel::Warn => _raw::ANDROID_LOG_WARN,
            DiagLevel::Error => _raw::ANDROID_LOG_ERROR,
            DiagLevel::Critical => _raw::ANDROID_LOG_FATAL,
        }
    }

    /// Writes `text` to Android's main log buffer.
    ///
    /// Returns `true` when the message was written and `false` when it was
    /// filtered by the Android logging configuration.
    #[macro_apply(crate::_android)]
    pub fn write(&self, level: DiagLevel, text: &CStr) -> bool {
        unsafe {
            _raw::__android_log_write(Self::priority(level), self.tag.as_ptr(), text.as_ptr()) == 1
        }
    }
    /// Writes UTF-8 `text` to Android's main log buffer.
    ///
    /// The diagnostic level is mapped to Android's closest native log priority.
    ///
    /// Returns `true` when the message was written
    /// and `false` when Android's logging configuration filtered it.
    ///
    /// # Errors
    ///
    /// Returns [`InvalidText::InteriorNul`] if `text` contains an interior NUL byte,
    /// or [`InvalidText::MismatchedCapacity`] if its length cannot be represented
    /// by Android's native logging interface.
    #[macro_apply(crate::_android)]
    pub fn write_str(&self, level: DiagLevel, text: &str) -> Result<bool, InvalidText> {
        if let Some(index) = text.as_bytes().iter().position(|&b| b == 0) {
            return Err(InvalidText::InteriorNul(index));
        }
        let len = match c_int::try_from(text.len()) {
            Ok(len) => len,
            Err(_) => {
                return Err(MismatchedCapacity::too_large(text.len(), c_int::MAX as usize).into());
            }
        };
        is! { text.is_empty(), return Ok(self.write(level, c"")) }
        let written = unsafe {
            _raw::__android_log_print(
                Self::priority(level),
                self.tag.as_ptr(),
                c"%.*s".as_ptr(),
                len,
                text.as_ptr().cast::<c_char>(),
            ) == 1
        };
        Ok(written)
    }
}

#[macro_apply(crate::_android)]
impl DiagOut for AndroidLog<'_> {
    type Error = InvalidText;

    fn diag(&mut self, level: DiagLevel, text: &str) -> Result<(), Self::Error> {
        self.write_str(level, text).map(|_| ())
    }
}
