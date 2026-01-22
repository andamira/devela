// devela_base_core::sys::log::const
//
//! Defines [`LoggerStatic`] and [`slog`].
//
// TOC
// - macro guard! (private)
// - struct LoggerStatic
// - macro slog!
//
// IMPROVE: log levels, in another array
// IMPROVE: configurable message eviction policy with state, internal enum.

use crate::{Slice, Str, is, lets, whilst};

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

#[doc = crate::_tags!(log)]
/// Fixed-capacity static logger with owned byte buffers.
///
/// Each logger is identified by its capacity (`CAP`) as the number of stored messages,
/// and each message’s maximum length (`MSG_LEN`).
///
/// Use the [`slog!`] macro to define, log, clear, or iterate its messages.
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

    /// Returns the number of logged messages.
    #[inline(always)]
    pub const fn count(&mut self) -> usize {
        guard![self enter "LoggerStatic::len()"];
        let len = self.len;
        guard![self leave];
        len
    }

    /// Whether the logger is full.
    #[must_use]
    #[inline(always)]
    pub const fn is_full(&mut self) -> bool {
        guard![self enter "LoggerStatic::is_full()"];
        let full = self.len == CAP;
        guard![self leave];
        full
    }

    /// Logs bytes from `msg`, truncating if longer than `MSG_LEN`.
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

    /// Returns the message and truncation count at index `i`, or `None` if out of bounds.
    ///
    /// # Features
    /// Uses the `unsafe_str` feature to skip duplicated UTF-8 validation.
    #[must_use]
    pub const fn get(&mut self, i: usize) -> Option<(&str, usize)> {
        guard![self enter "LoggerStatic::get()"];
        is! { i >= self.len; { guard![self leave]; return None; }}

        let len = self.lens[i];
        let leftover = self.leftover[i];
        let msg = Slice::range_to(&self.buf[i], len);

        #[cfg(any(base_safe_text, not(feature = "unsafe_str")))]
        let s = crate::unwrap![ok Str::from_utf8(msg)];
        #[cfg(all(not(base_safe_text), feature = "unsafe_str"))]
        // SAFETY: we ensure to always contain valid UTF-8
        let s = unsafe { Str::from_utf8_unchecked(msg) };

        guard![self leave];
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
        guard![self enter "LoggerStatic::log_bytes()"];
        for (i, msg) in self.buf[..self.len].iter().enumerate() {
            let len = self.lens[i];
            let leftover = self.leftover[i];

            #[cfg(any(base_safe_text, not(feature = "unsafe_str")))]
            let s = crate::unwrap![ok Str::from_utf8(&msg[..len])];
            #[cfg(all(not(base_safe_text), feature = "unsafe_str"))]
            // SAFETY: we ensure to always contain valid UTF-8
            let s = unsafe { Str::from_utf8_unchecked(&msg[..len]) };

            f(i, s, leftover);
        }
        guard![self leave];
    }

    /// Returns `true` if any logged message was truncated.
    #[must_use]
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
        lets![mut count = 0,  mut total = 0];
        whilst! { i in 0..self.len; {
            let lost = self.leftover[i];
            is! { lost > 0; { count += 1; total += lost; }}
        }}
        guard![self leave];
        (count, total)
    }
}

#[doc = crate::_tags!(log)]
/// Static global logger macro, compile-time friendly.
///
/// Defines and operates on [`LoggerStatic`] instances for fixed-capacity logging.
/// Each logger is identified by its `(CAP, MSG_LEN)` pair and may optionally have
/// an extra identifier for multiple independent instances.
///
/// A `new` static logger can be given custom attributes and visibility.
///
/// See `LoggerStatic` for details on storage, safety, and const behavior.
///
/// # Overview
/// - `slog!([vis] new [id:]CAP+LEN)` — define a static logger.
/// - `slog!([id:]CAP+LEN "...")` — log a formatted message.
/// - `slog!(clear [id:]CAP+LEN)` — clear all messages.
/// - `slog!(get [id:]CAP+LEN, idx)` — get some message at idx.
/// - `slog!(for_each [id:]CAP+LEN |idx, msg, trunc| { ... })` — iterate logged messages.
/// - `slog!(any_truncated [id:]CAP+LEN)` — check if any message was truncated.
/// - `slog!(truncation_stats [id:]CAP+LEN)` — return truncation count and total lost bytes.
///
/// # Example
/// ```
/// # use devela_base_core::slog;
/// slog!(new 4+32);
/// slog!(4+32 "init ok");
/// slog!(4+32 "processing step ", %2u8, ".");
/// # #[cfg(feature = "__std")]
/// slog!(for_each 4+32 |idx, msg, _trunc| println!("[{idx}] {msg}"));
/// slog!(clear 4+32);
///
/// // with an extra identifier
/// slog!(#[doc(hidden)] pub new log1:4+32); // with custom attributes & visibility
/// slog!(log1:4+32 "hello world");
/// assert_eq![slog!(get log1:4+32, 0), Some(("hello world", 0))];
/// slog!(clear log1:4+32);
/// ```
#[macro_export]
#[cfg_attr(cargo_primary_package, doc(hidden))]
macro_rules! slog {
    (
    /* public API*/

    // Define a static logger.
    $(#[$attrs:meta])*
    $vis:vis new $($id:ident :)? $CAP:literal + $LEN:literal) => { $crate::paste! {
        $crate::slog![@$(#[$attrs])* $vis new
            $($id:)? $CAP+$LEN,
            // NOTE: getting the ident using the macro doesn't seem compatible with paste!
            // slog![@static_ident $($id:)?$CAP+$LEN],
            // slog![@fn_ident $($id:)?$CAP+$LEN],
            [<__LOGGER_ $($id _)?$CAP _$LEN>], // $static
            [<__logger_ $($id _)?$CAP _$LEN>], // $fn
        ];
    }};
    (
    // Clear all logs.
    clear $($id:ident :)? $CAP:literal + $LEN:literal) => {
        $crate::slog!(@get $($id:)? $CAP + $LEN).clear();
    };
    (
    // Return the number of messages.
    count $($id:ident :)? $CAP:literal + $LEN:literal) => {
        $crate::slog!(@get $($id:)? $CAP + $LEN).count();
    };
    (
    // Whether the logger is full.
    is_full $($id:ident :)? $CAP:literal + $LEN:literal) => {
        $crate::slog!(@get $($id:)? $CAP + $LEN).is_full();
    };
    (
    // Log message with formatted arguments.
    $($id:ident :)? $CAP:literal + $LEN:literal $($fmt:tt)+) => {{
        let mut buf = [0u8; $LEN];
        let mut pos = 0;
        $crate::fmtcat!(buf, pos, $($fmt)+);
        let slice = $crate::Slice::range_to(&buf, pos);
        $crate::slog!(@get $($id:)? $CAP + $LEN).log_bytes(slice);
    }};
    (
    // Retrieve a specific message by index.
    get $($id:ident :)? $CAP:literal + $LEN:literal, $index:expr) => {
        $crate::slog!(@get $($id :)? $CAP + $LEN).get($index)
    };
    (
    // Run a closure for each log message.
    for_each $($id:ident :)? $CAP:literal + $LEN:literal $closure:expr) => {
        $crate::slog!(@get $($id:)? $CAP + $LEN).for_each($closure);
    };
    (
    // Whether if any message was truncated.
    any_truncated $($id:ident :)? $CAP:literal + $LEN:literal) => {
        $crate::slog!(@get $($id:)? $CAP + $LEN).any_truncated();
    };
    (
    // Returns `(count, total_lost_bytes)` for truncated messages.
    truncation_stats $($id:ident :)? $CAP:literal + $LEN:literal) => {
        $crate::slog!(@get $($id:)? $CAP + $LEN).truncation_stats();
    };
    (
    // Returns the name name of the global static as a static string slice.
    static_name $($id:ident :)? $CAP:literal + $LEN:literal) => { $crate::paste! {
        stringify!{ $crate::slog![@static_ident $($id :)? $CAP+$LEN] }
    }};
    // (
    // // Returns name of the global fn as a static string slice.
    // fn_name $($id:ident :)? $CAP:literal + $LEN:literal) => { $crate::paste! {
    //     stringify!{ $crate::slog![@fn_ident $($id :)? $CAP+$LEN] }
    // }};
    (
    /* private API */
    @$(#[$attrs:meta])*
    $vis:vis new $($id:ident :)? $CAP:literal + $LEN:literal,
    $static:ident, $fn:ident $(,)?) => { $crate::paste! {
        $(#[$attrs])*
        #[doc = "\nA single-thread global static logger `" $($id ":" )? $CAP "+" $LEN "`."]
        ///
        /// Query it with the [`slog!`] macro.
        /// ```
        /// # use devela_base_core::*;
        /// # #[cfg(feature = "__std")]
        #[doc = "slog![for_each " $($id ":" )? $CAP "+" $LEN " |i, s, _| println!(\"[{i}] {s}\")];"]
        /// ```
        // #[doc = "See also the fn `" $fn "()`."]
        #[allow(non_upper_case_globals, reason = "case-sensitive $id")]
        $vis static mut $static: $crate::LoggerStatic<$CAP, $LEN> = $crate::LoggerStatic::new();

        #[doc(hidden)] // the unsafe fn accessor is always hidden
        $(#[$attrs])*
        #[doc = "Returns a mutable reference to the global static [`" $static "`]."]
        ///
        /// # Safety
        /// The caller must ensure single-threaded discipline when mutating the returned reference.
        #[inline(always)]
        $vis const unsafe fn $fn() -> &'static mut $crate::LoggerStatic<$CAP, $LEN> {
            #[allow(static_mut_refs, reason = "accessing the single-thread static logger instance")]
            // SAFETY: user upholds single-threaded access to this static instance.
            unsafe { &mut $static }
        }
    }};
    // returns the global static reference
    (@get $($id:ident :)? $CAP:literal + $LEN:literal) => {{
        unsafe { $crate::slog![@fn_ident $($id :)? $CAP+$LEN]() }
    }};
    // returns the identifier of the global static
    (@static_ident $($id:ident :)? $CAP:literal + $LEN:literal) => { $crate::paste! {
        [<__LOGGER_ $($id _)? $CAP _ $LEN>]
    }};
    // returns the identifier of the global fn
    (@fn_ident $($id:ident :)? $CAP:literal + $LEN:literal) => { $crate::paste! {
        [<__logger_ $($id _)? $CAP _ $LEN>]
    }};
    // import the static
    (@$(#[$attr:meta])*
     $vis:vis use static: $($id:ident :)? $CAP:literal+$LEN:literal in $path:path) => {
        // NOTE: we can't call a macro to get the ident at the end of the path.
        $crate::paste! { $(#[$attr])* $vis use $path::[<__LOGGER_ $($id _)? $CAP _ $LEN>]; }
    };
    // import the fn
    (@$(#[$attr:meta])*
     $vis:vis use fn: $($id:ident :)? $CAP:literal+$LEN:literal in $path:path) => {
        $crate::paste! { $(#[$attr])* $vis use $path::[<__logger_ $($id _)? $CAP _ $LEN>]; }
    };
}
#[doc(inline)]
pub use slog;
