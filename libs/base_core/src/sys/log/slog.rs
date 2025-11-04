// devela_base_core::sys::log::const
//
//! Defines [`LoggerStatic`] and [`slog`].
//

use crate::{Slice, Str, is, lets, unwrap, whilst};

/// Fixed-capacity static logger with owned byte buffers.
///
/// Each logger is identified by its capacity (`CAP`) — the number of
/// stored messages — and each message’s maximum length (`MSG_LEN`).
///
/// Use the [`slog!`] macro to define, log, clear, or iterate its messages.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct LoggerStatic<const CAP: usize, const MSG_LEN: usize> {
    buf: [[u8; MSG_LEN]; CAP],
    lens: [usize; CAP],
    leftover: [usize; CAP], // 0 = no truncation
    len: usize,
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
        }
    }

    /// Clears all the stored messages.
    #[inline(always)]
    pub const fn clear(&mut self) {
        self.len = 0;
    }

    /// Logs bytes from `msg`, truncating if longer than `MSG_LEN`.
    #[inline(always)]
    pub const fn log_bytes(&mut self, msg: &[u8]) {
        if self.len < CAP {
            let written = Slice::copy_utf8_into(&mut self.buf[self.len], 0, msg);
            let remaining = msg.len().saturating_sub(written);
            self.lens[self.len] = written;
            self.leftover[self.len] = remaining;
            self.len += 1;
        }
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
    pub fn for_each<F: FnMut(usize, &str, usize)>(&self, mut f: F) {
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
    }

    /// Returns `true` if any logged message was truncated.
    #[must_use]
    #[inline(always)]
    pub const fn any_truncated(&self) -> bool {
        whilst! { i in 0..self.len; {
            is![self.leftover[i] > 0; return true];
        }}
        false
    }

    /// Returns `(count, total_lost_bytes)` for truncated messages.
    #[must_use]
    pub const fn truncation_stats(&self) -> (usize, usize) {
        lets![mut count = 0,  mut total = 0, mut i = 0];
        while i < self.len {
            let lost = self.leftover[i];
            if lost > 0 {
                count += 1;
                total += lost;
            }
            i += 1;
        }
        (count, total)
    }

    // MAYBE
    // /// Prints all messages (runtime & std only).
    // ///
    // /// # Features
    // /// - Needs the `__std` feature to actually print.
    // /// - Uses the `unsafe_str` feature to skip duplicated validation checks.
    // #[allow(unused_variables, reason = "__std feature-gate")]
    // pub fn print(&self, name: &str) {
    //     self.for_each(|i, s| { crate::__std! { println!("{name}[{i}]: {s}");} } );
    // }
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
/// ```
#[macro_export]
macro_rules! slog {
    ( // Define a static logger.
    new $CAP:literal + $LEN:literal) => { $crate::paste! {
        #[doc(hidden)]
        pub static mut [<LOGGER $CAP _ $LEN>]: $crate::LoggerStatic<$CAP, $LEN>
            = $crate::LoggerStatic::new();

        /// Returns a mutable reference to the global static logger instance.
        ///
        /// # Safety
        /// The caller must ensure single-threaded discipline when mutating the returned reference.
        #[inline(always)]
        pub const fn [<logger_ $CAP _ $LEN _static_ref>]()
            -> &'static mut $crate::LoggerStatic<$CAP, $LEN> {
            #[allow(static_mut_refs)]
            // SAFETY: user upholds single-threaded access to this static instance.
            unsafe { &mut [<LOGGER $CAP _ $LEN>] }
        }
    }};

    ( // Clear all logs.
    clear $CAP:literal + $LEN:literal) => {
        $crate::slog!(@get $CAP + $LEN).clear();
    };
    ( // Log message with formatted arguments.
    $CAP:literal + $LEN:literal $($fmt:tt)+) => {{
        let mut buf = [0u8; $LEN];
        let mut pos = 0;
        $crate::fmtcat!(buf, pos, $($fmt)+);
        let slice = $crate::Slice::range_to(&buf, pos);
        $crate::slog!(@get $CAP + $LEN).log_bytes(slice);
    }};
    ( // Run a closure for each log message.
    for_each $CAP:literal + $LEN:literal $closure:expr) => {
        $crate::slog!(@get $CAP + $LEN).for_each($closure);
    };
    ( // Returns true if any message was truncated.
    any_truncated $CAP:literal + $LEN:literal) => {
        $crate::slog!(@get $CAP + $LEN).any_truncated();
    };
    ( // Returns `(count, total_lost_bytes)` for truncated messages.
    truncation_stats $CAP:literal + $LEN:literal) => {
        $crate::slog!(@get $CAP + $LEN).truncation_stats();
    };
    // MAYBE
    // ( // Print logs.
    // static print $CAP:literal + $LEN:literal $name:literal) => {
    //     $crate::LoggerStatic::<$CAP, $LEN>::global().print($name);
    // };

    // inner helper to get the global static reference
    (@get $CAP:literal + $LEN:literal) => {{
        $crate::paste! { [<logger_ $CAP _ $LEN _static_ref>]() }
    }};
}
pub use slog;
