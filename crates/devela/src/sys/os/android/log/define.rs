//
//! Defines [`AndroidLog`] and [`AndroidLogPriority`].
//

use crate::CStr;

#[cfg(target_os = "android")]
use super::_raw;

#[doc = crate::_tags!(platform log)]
/// Android native log priority.
#[doc = crate::_doc_meta!{
    location("sys/os/android/", enum AndroidLogPriority),
}]
#[must_use]
#[repr(i32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AndroidLogPriority {
    /// Verbose diagnostic logging.
    Verbose = 2,
    /// Debug diagnostic logging.
    Debug = 3,
    /// Informational logging.
    Info = 4,
    /// Warning logging.
    Warn = 5,
    /// Error logging.
    Error = 6,
    /// Fatal-condition logging.
    Fatal = 7,
}

#[doc = crate::_tags!(platform log)]
/// Android native logger with a fixed log tag.
#[doc = crate::_doc_meta!{
    location("sys/os/android", struct AndroidLog),
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

    /// Writes `text` to Android's main log buffer.
    ///
    /// Returns `true` when the message was written and `false` when it was
    /// filtered by the Android logging configuration.
    #[cfg(target_os = "android")]
    pub fn write(&self, priority: AndroidLogPriority, text: &CStr) -> bool {
        unsafe { _raw::__android_log_write(priority as _, self.tag.as_ptr(), text.as_ptr()) == 1 }
    }
}
