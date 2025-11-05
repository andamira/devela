// devela_base_core::sys::log::const
//
//! Defines [`LoggerStatic`] and [`slog`].
//
// TOC
// - macro guard! (private)
// - struct LoggerStatic
// - macro slog!

use crate::{Slice, Str, is, lets, unwrap, whilst};

/// Internal helper to set the re-entrancy guard with a custom panic message.
///
/// Provides `enter` and `leave` arms for setting and clearing the guard.
///
/// # Panics
/// Panics on `enter` if the logger is already in use,
/// reporting the calling method's name in the message.
macro_rules! guard {
    ($self:ident enter $msg:literal) => { is![$self.guard != 0; panic!($msg)]; $self.guard = 1; };
    ($self:ident leave) => { $self.guard = 0; };
}

/// Fixed-capacity static logger with owned byte buffers.
///
/// Each logger is identified by its capacity (`CAP`) — the number of
/// stored messages — and each message’s maximum length (`MSG_LEN`).
///
/// Use the [`slog!`] macro to define, log, clear, or iterate its messages.
///
/// # Safety
/// Employs a single-byte, non-atomic guard as a const-safe soft check against
/// re-entrant or concurrent access. While not providing full thread safety,
/// it can detect most accidental overlaps in single-threaded or unsynchronized
/// contexts, serving as a last-resort diagnostic aid under the assumption of
/// disciplined single-threaded use.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct LoggerStatic<const CAP: usize, const MSG_LEN: usize> {
    buf: [[u8; MSG_LEN]; CAP],
    lens: [usize; CAP],
    leftover: [usize; CAP], // 0 = no truncation
    len: usize,
    guard: i8, // 0 = free, 1 = in use
}

impl<const CAP: usize, const MSG_LEN: usize> LoggerStatic<CAP, MSG_LEN> {
    /// Constructs a new static logger.
    #[inline(always)]
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
    #[inline(always)]
    pub const fn clear(&mut self) {
        guard![self enter "LoggerStatic::clear()"];
        self.len = 0;
        guard![self leave];
    }

    /// Logs bytes from `msg`, truncating if longer than `MSG_LEN`.
    #[inline(always)]
    pub const fn log_bytes(&mut self, msg: &[u8]) {
        guard![self enter "LoggerStatic::log_bytes()"];
        if self.len < CAP {
            let written = Slice::copy_utf8_into(&mut self.buf[self.len], 0, msg);
            let remaining = msg.len().saturating_sub(written);
            self.lens[self.len] = written;
            self.leftover[self.len] = remaining;
            self.len += 1;
        }
        guard![self leave];
    }

    /// Whether the logger is full.
    #[inline(always)]
    pub const fn is_full(&mut self) -> bool {
        guard![self enter "LoggerStatic::is_full()"];
        let full = self.len == CAP;
        guard![self leave];
        full
    }

    /// Runs the provided closure for each message.
    ///
    /// The closure receives:
    /// - `i`: index of the message
    /// - `s`: string slice (truncated if necessary)
    /// - `truncated`: number of bytes that did not fit
    ///
    /// # Features
    /// Uses the `unsafe_str` feature to skip duplicated validation checks.
    pub fn for_each<F: FnMut(usize, &str, usize)>(&mut self, mut f: F) {
        guard![self enter "LoggerStatic::log_bytes()"];
        for (i, msg) in self.buf[..self.len].iter().enumerate() {
            let len = self.lens[i];
            let leftover = self.leftover[i];

            #[cfg(any(base_safe_text, not(feature = "unsafe_str")))]
            let s = unwrap![ok Str::from_utf8(&msg[..len])];
            #[cfg(all(not(base_safe_text), feature = "unsafe_str"))]
            // SAFETY: we ensure to always contain valid UTF-8
            let s = unsafe { Str::from_utf8_unchecked(&msg[..len]) };

            f(i, s, leftover);
        }
        guard![self leave];
    }

    /// Returns `true` if any logged message was truncated.
    #[must_use]
    #[inline(always)]
    pub const fn any_truncated(&mut self) -> bool {
        guard![self enter "LoggerStatic::any_truncated()"];
        whilst! { i in 0..self.len; {
            is![self.leftover[i] > 0; { guard![self leave]; return true; }];

        }}
        guard![self leave];
        false
    }

    /// Returns `(count, total_lost_bytes)` for truncated messages.
    #[must_use]
    pub const fn truncation_stats(&mut self) -> (usize, usize) {
        guard![self enter "LoggerStatic::truncation_stats()"];
        lets![mut count = 0,  mut total = 0, mut i = 0];
        while i < self.len {
            let lost = self.leftover[i];
            if lost > 0 {
                count += 1;
                total += lost;
            }
            i += 1;
        }
        guard![self leave];
        (count, total)
    }
}

/// Static global logger macro, compile-time friendly.
///
/// # Example
/// ```
/// # use devela_base_core::slog;
/// slog!(new 4+32);
/// slog!(4+32 "init ok");
/// slog!(4+32 "processing step ", %2u8, ".");
/// # #[cfg(feature = "__std")]
/// slog!(for_each 4+32 |i, s, _s| println!("[{i}] {s}"));
/// slog!(clear 4+32);
///
/// // Supports an extra identifier
/// slog!(new id0:4+32);
/// slog!(id0:4+32 "hello world");
/// slog!(clear id0:4+32);
/// ```
#[macro_export]
macro_rules! slog {
    ( // Define a static logger.
    new $($id:ident :)? $CAP:literal + $LEN:literal) => { $crate::paste! {
        #[doc(hidden)]
        #[allow(non_upper_case_globals, reason = "case-sensitive $id")]
        pub static mut [<LOGGER_ $($id _)? $CAP _ $LEN>]: $crate::LoggerStatic<$CAP, $LEN>
            = $crate::LoggerStatic::new();

        /// Returns a mutable reference to the global static logger instance.
        ///
        /// # Safety
        /// The caller must ensure single-threaded discipline when mutating the returned reference.
        #[inline(always)]
        pub const fn [<logger_ $($id _)? $CAP _ $LEN _static_ref>]()
            -> &'static mut $crate::LoggerStatic<$CAP, $LEN> {
            #[allow(static_mut_refs, reason = "accessing the single-thread static logger instance")]
            // SAFETY: user upholds single-threaded access to this static instance.
            unsafe { &mut [<LOGGER_ $($id _)? $CAP _ $LEN>] }
        }
    }};

    ( // Clear all logs.
    clear $($id:ident :)? $CAP:literal + $LEN:literal) => {
        $crate::slog!(@get $($id:)? $CAP + $LEN).clear();
    };
    ( // Log message with formatted arguments.
    $($id:ident :)? $CAP:literal + $LEN:literal $($fmt:tt)+) => {{
        let mut buf = [0u8; $LEN];
        let mut pos = 0;
        $crate::fmtcat!(buf, pos, $($fmt)+);
        let slice = $crate::Slice::range_to(&buf, pos);
        $crate::slog!(@get $($id:)? $CAP + $LEN).log_bytes(slice);
    }};
    ( // Returns true if the logger is full.
    is_full $($id:ident :)? $CAP:literal + $LEN:literal) => {
        $crate::slog!(@get $($id:)? $CAP + $LEN).is_full();
    };
    ( // Run a closure for each log message.
    for_each $($id:ident :)? $CAP:literal + $LEN:literal $closure:expr) => {
        $crate::slog!(@get $($id:)? $CAP + $LEN).for_each($closure);
    };
    ( // Returns true if any message was truncated.
    any_truncated $($id:ident :)? $CAP:literal + $LEN:literal) => {
        $crate::slog!(@get $($id:)? $CAP + $LEN).any_truncated();
    };
    ( // Returns `(count, total_lost_bytes)` for truncated messages.
    truncation_stats $($id:ident :)? $CAP:literal + $LEN:literal) => {
        $crate::slog!(@get $($id:)? $CAP + $LEN).truncation_stats();
    };
    // inner helper to get the global static reference
    (@get $($id:ident :)? $CAP:literal + $LEN:literal) => {{
        $crate::paste! { [<logger_ $($id _)? $CAP _ $LEN _static_ref>]() }
    }};
}
pub use slog;
