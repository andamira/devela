//
//! Defines [`LoggerStatic`].
//

use crate::{Slice, Str, is, lets, whilst};

/// Internal helper to set the re-entrancy guard with a custom panic message.
///
/// Provides `enter` and `leave` arms for setting and clearing the guard.
///
/// # Panics
/// Panics on `enter` if the logger is already in use,
/// reporting the calling method's name in the message.
macro_rules! _slog_guard {
    ($self:ident enter $msg:literal) => {
        is![$self.guard != 0, panic!($msg)];
        $self.guard = 1;
    };
    ($self:ident leave) => {
        $self.guard = 0;
    };
}

#[doc = crate::_tags!(log)]
/// Fixed-capacity static logger with owned byte buffers.
#[doc = crate::_doc_meta!{
    location("sys/log", struct LoggerStatic),
    #[cfg(target_pointer_width = "32")]
    test_size_of(LoggerStatic<8, 16> = 200|1600; niche !Option),
    #[cfg(target_pointer_width = "64")]
    test_size_of(LoggerStatic<8, 16> = 272|2176; niche !Option),
}]
/// Each logger is identified by its capacity (`CAP`) as the number of stored messages,
/// and each message's maximum length (`MSG_LEN`).
///
/// Use the [`slog!`][crate::slog] macro to define, log, clear, or iterate its messages.
///
/// # Safety
/// Employs a single-byte, non-atomic guard as a const-safe soft check against
/// re-entrant or concurrent access.
///
/// While not providing full thread safety, it can detect most accidental overlaps
/// in single-threaded or unsynchronized contexts, serving as a last-resort diagnostic
/// aid under the assumption of disciplined single-threaded use.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct LoggerStatic<const CAP: usize, const MSG_LEN: usize> {
    buf: [[u8; MSG_LEN]; CAP],
    lens: [usize; CAP],
    leftover: [usize; CAP], // 0 = no truncation
    len: usize,
    guard: i8, // 0 = free, 1 = in use
}

impl<const CAP: usize, const MSG_LEN: usize> Default for LoggerStatic<CAP, MSG_LEN> {
    fn default() -> Self {
        Self::new()
    }
}

impl<const CAP: usize, const MSG_LEN: usize> LoggerStatic<CAP, MSG_LEN> {
    /// Constructs a new static logger.
    pub const fn new() -> Self {
        Self {
            buf: [[0; MSG_LEN]; CAP],
            lens: [0; CAP],
            leftover: [0; CAP],
            len: 0,
            guard: 0,
        }
    }

    /// Clears all the stored messages.
    pub const fn clear(&mut self) {
        _slog_guard![self enter "LoggerStatic::clear()"];
        self.len = 0;
        _slog_guard![self leave];
    }

    /// Returns the number of logged messages.
    pub const fn count(&mut self) -> usize {
        _slog_guard![self enter "LoggerStatic::len()"];
        let len = self.len;
        _slog_guard![self leave];
        len
    }

    /// Whether the logger is full.
    #[must_use]
    pub const fn is_full(&mut self) -> bool {
        _slog_guard![self enter "LoggerStatic::is_full()"];
        let full = self.len == CAP;
        _slog_guard![self leave];
        full
    }

    /// Logs bytes from `msg`, truncating if longer than `MSG_LEN`.
    pub const fn log_bytes(&mut self, msg: &[u8]) {
        _slog_guard![self enter "LoggerStatic::log_bytes()"];
        if self.len < CAP {
            let written = Slice::copy_utf8_into(&mut self.buf[self.len], 0, msg);
            let remaining = msg.len().saturating_sub(written);
            self.lens[self.len] = written;
            self.leftover[self.len] = remaining;
            self.len += 1;
        }
        _slog_guard![self leave];
    }

    /// Returns the message and truncation count at index `i`, or `None` if out of bounds.
    ///
    /// # Features
    /// Uses the `unsafe_str` feature to skip duplicated UTF-8 validation.
    #[must_use]
    pub const fn get(&mut self, i: usize) -> Option<(&str, usize)> {
        _slog_guard![self enter "LoggerStatic::get()"];
        is! { i >= self.len, { _slog_guard![self leave]; return None; }}

        let len = self.lens[i];
        let leftover = self.leftover[i];
        let msg = Slice::range_to(&self.buf[i], len);

        cfg_select! { all(feature = "unsafe_str", not(feature = "safe_text")) => {
            // SAFETY: we ensure to always contain valid UTF-8
            let s = unsafe { Str::from_utf8_unchecked(msg) };
        } _ => {
            let s = crate::unwrap![ok Str::from_utf8(msg)];
        }}
        _slog_guard![self leave];
        Some((s, leftover))
    }

    /// Runs the provided closure for each message in run-time.
    ///
    /// The closure receives:
    /// - `i`: index of the message
    /// - `s`: string slice (truncated if necessary)
    /// - `truncated`: number of bytes that did not fit
    ///
    /// # Features
    /// Uses the `unsafe_str` feature to skip duplicated validation checks.
    pub fn for_each<F: FnMut(usize, &str, usize)>(&mut self, mut f: F) {
        _slog_guard![self enter "LoggerStatic::log_bytes()"];
        for (i, msg) in self.buf[..self.len].iter().enumerate() {
            let len = self.lens[i];
            let leftover = self.leftover[i];

            cfg_select! { all(feature = "unsafe_str", not(feature = "safe_text")) => {
                // SAFETY: we ensure to always contain valid UTF-8
                let s = unsafe { Str::from_utf8_unchecked(&msg[..len]) };
            } _ => {
                let s = crate::unwrap![ok Str::from_utf8(&msg[..len])];
            }}
            f(i, s, leftover);
        }
        _slog_guard![self leave];
    }

    /// Returns `true` if any logged message was truncated.
    #[must_use]
    pub const fn any_truncated(&mut self) -> bool {
        _slog_guard![self enter "LoggerStatic::any_truncated()"];
        whilst! { i in 0..self.len; {
            is![self.leftover[i] > 0, { _slog_guard![self leave]; return true; }];
        }}
        _slog_guard![self leave];
        false
    }

    /// Returns `(count, total_lost_bytes)` for truncated messages.
    #[must_use]
    pub const fn truncation_stats(&mut self) -> (usize, usize) {
        _slog_guard![self enter "LoggerStatic::truncation_stats()"];
        lets![mut count = 0,  mut total = 0];
        whilst! { i in 0..self.len; {
            let lost = self.leftover[i];
            is! { lost > 0, { count += 1; total += lost; }}
        }}
        _slog_guard![self leave];
        (count, total)
    }
}
