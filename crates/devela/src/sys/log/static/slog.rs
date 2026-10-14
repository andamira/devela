//
//! Defines [`slog`].
//

#[doc = crate::_tags!(log)]
/// Static global logger macro, compile-time friendly.
#[doc = crate::_doc_meta!{
    location("sys/log", macro slog),
}]
/// Defines and operates on [`LoggerStatic`] instances for fixed-capacity logging.
/// Each logger is identified by its `(CAP, MSG_LEN)` pair and may optionally have
/// an extra identifier for multiple independent instances.
///
/// A `new` static logger can be given custom attributes and visibility.
///
/// See `LoggerStatic` for details on storage, safety, and const behavior.
///
/// [`LoggerStatic`]: crate::LoggerStatic
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
/// # Examples
/// ```
/// use devela::slog;
///
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
#[cfg_attr(nightly_doc, doc(cfg(feature = "unsafe_sync")))]
#[cfg(feature = "unsafe_sync")]
macro_rules! slog· {
    (
    /* public API*/

    // Define a static logger.
    $(#[$attrs:meta])*
    $vis:vis new $($id:ident :)? $CAP:literal + $LEN:literal) => { $crate::paste! {
        $crate::__slog![%$(#[$attrs])* $vis new
            $($id:)? $CAP+$LEN,
            // NOTE: getting the ident using the macro doesn't seem compatible with paste!
            // __slog![%static_ident $($id:)?$CAP+$LEN],
            // __slog![%fn_ident $($id:)?$CAP+$LEN],
            [<__LOGGER_ $($id _)?$CAP _$LEN>], // $static
            [<__logger_ $($id _)?$CAP _$LEN>], // $fn
        ];
    }};
    (
    // Clear all logs.
    clear $($id:ident :)? $CAP:literal + $LEN:literal) => {
        $crate::__slog![%get $($id:)? $CAP + $LEN].clear();
    };
    (
    // Return the number of messages.
    count $($id:ident :)? $CAP:literal + $LEN:literal) => {
        $crate::__slog![%get $($id:)? $CAP + $LEN].count();
    };
    (
    // Whether the logger is full.
    is_full $($id:ident :)? $CAP:literal + $LEN:literal) => {
        $crate::__slog![%get $($id:)? $CAP + $LEN].is_full();
    };
    (
    // Log message with formatted arguments.
    $($id:ident :)? $CAP:literal + $LEN:literal $($fmt:tt)+) => {{
        let mut buf = [0u8; $LEN];
        let mut pos = 0;
        $crate::fmtcat!(buf, pos, $($fmt)+);
        let slice = $crate::Slice::range_to(&buf, pos);
        $crate::__slog![%get $($id:)? $CAP + $LEN].log_bytes(slice);
    }};
    (
    // Retrieve a specific message by index.
    get $($id:ident :)? $CAP:literal + $LEN:literal, $index:expr) => {
        $crate::__slog![%get $($id :)? $CAP + $LEN].get($index)
    };
    (
    // Run a closure for each log message.
    for_each $($id:ident :)? $CAP:literal + $LEN:literal $closure:expr) => {
        $crate::__slog![%get $($id:)? $CAP + $LEN].for_each($closure);
    };
    (
    // Whether if any message was truncated.
    any_truncated $($id:ident :)? $CAP:literal + $LEN:literal) => {
        $crate::__slog![%get $($id:)? $CAP + $LEN].any_truncated();
    };
    (
    // Returns `(count, total_lost_bytes)` for truncated messages.
    truncation_stats $($id:ident :)? $CAP:literal + $LEN:literal) => {
        $crate::__slog![%get $($id:)? $CAP + $LEN].truncation_stats();
    };
    (
    // Returns the name name of the global static as a static string slice.
    static_name $($id:ident :)? $CAP:literal + $LEN:literal) => { $crate::paste! {
        stringify!{ $crate::__slog![%static_ident $($id :)? $CAP+$LEN] }
    }};
    // (
    // // Returns name of the global fn as a static string slice.
    // fn_name $($id:ident :)? $CAP:literal + $LEN:literal) => { $crate::paste! {
    //     stringify!{ $crate::__slog![%fn_ident $($id :)? $CAP+$LEN] }
    // }};
}

// no-op macro placeholder
#[doc = crate::_tags!(fake log)]
/// Static global logger macro, compile-time friendly.
#[doc = crate::_doc_meta!{
    location("sys/log", macro slog),
}]
/// This inert implementation preserves call-site ergonomics
/// when the logging backend is not enabled.
///
/// # Features
/// Enable the real implementation with the `unsafe_sync` feature.
#[cfg_attr(not(feature = "__docs_internal"), doc(hidden))]
#[cfg_attr(cargo_primary_package, doc(hidden))]
#[cfg_attr(nightly_doc, doc(cfg(feature = "unsafe_sync")))]
#[cfg(not(feature = "unsafe_sync"))]
#[macro_export]
macro_rules! slog· {
    ($($tt:tt)*) => {};
}

#[doc(inline)]
pub use slog· as slog;

/* hidden internals */

#[doc(hidden)]
#[macro_export]
#[cfg(feature = "unsafe_sync")]
macro_rules! __slog· {
    (% $(#[$attrs:meta])*
     $vis:vis new $($id:ident :)? $CAP:literal + $LEN:literal,
     $static:ident, $fn:ident $(,)?) => { $crate::paste! {
        $(#[$attrs])*
        #[doc = "\n\nA single-thread global static logger `" $($id ":" )? $CAP "+" $LEN "`."]
        ///
        /// Query it with the [`slog!`] macro, like this:
        /// ```
        /// use devela::*;
        ///
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
        $vis const unsafe fn $fn() -> &'static mut $crate::LoggerStatic<$CAP, $LEN> {
            #[allow(static_mut_refs, reason = "accessing the single-thread static logger instance")]
            // SAFETY: user upholds single-threaded access to this static instance.
            unsafe { &mut $static }
        }
    }};
    // returns the global static reference
    (% get $($id:ident :)? $CAP:literal + $LEN:literal) => {{
        unsafe { $crate::__slog![%fn_ident $($id :)? $CAP+$LEN]() }
    }};
    // returns the identifier of the global static
    (% static_ident $($id:ident :)? $CAP:literal + $LEN:literal) => { $crate::paste! {
        [<__LOGGER_ $($id _)? $CAP _ $LEN>]
    }};
    // returns the identifier of the global fn
    (% fn_ident $($id:ident :)? $CAP:literal + $LEN:literal) => { $crate::paste! {
        [<__logger_ $($id _)? $CAP _ $LEN>]
    }};
    // import the static
    (% $(#[$attr:meta])*
     $vis:vis use static: $($id:ident :)? $CAP:literal+$LEN:literal in $path:path) => {
        // NOTE: we can't call a macro to get the ident at the end of the path.
        $crate::paste! { $(#[$attr])* $vis use $path::[<__LOGGER_ $($id _)? $CAP _ $LEN>]; }
    };
    // import the fn
    (% $(#[$attr:meta])*
     $vis:vis use fn: $($id:ident :)? $CAP:literal+$LEN:literal in $path:path) => {
        $crate::paste! { $(#[$attr])* $vis use $path::[<__logger_ $($id _)? $CAP _ $LEN>]; }
    };
}

// no-op macro placeholder
#[doc(hidden)]
#[macro_export]
#[cfg(not(feature = "unsafe_sync"))]
macro_rules! __slog· {
    ($($tt:tt)*) => {};
}

pub use __slog· as __slog;
