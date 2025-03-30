// devela::ui::front::term::ansi::macro
//
//! Defines the [`ansi!`] macro.
//
// NOTES:
// - features are in sync with /src/sys/os/print/mod.rs.
// - different macros are necessary to avoid evaluating the feature-bounds on user time.
// - versions differ only in having support for printing, and in the Ansi print method called.

crate::CONST! {
    DOC_ANSI = r#"Concatenates or prints [`Ansi` escape codes][0].

- the `b:` arm accepts only static arguments to commands and returns [`&[u8]`](slice).
- the `s:` arm accepts only static arguments to commands and returns [`&str`].
- the `p:` arm accepts only static arguments to commands and prints to stdout. `*`
- the `@p:` arm accepts dynamic arguments to commands and prints to stdout. `*`

## Features
`*` Printing is supported only when any of the following set of features are enabled:
- `linux`, `unsafe_syscall`. (takes preference)
- `std`.

## Notes
- commands are case-insensitive.
- the list of commands can be separated by spaces and/or commas.
- commands without parenthesis refer to [`Ansi`][crate::Ansi] associated constants.
- commands with parenthesis refers to `Ansi` associated functions.
- only commands that return an array are supported (not `*_N` versions).
- the static arms make use of the [`join!`][crate::join] macro for concatenation.
- the print arms calls the apropriate `Ansi` [`print`][crate::Ansi::print] method variant.

# Example
```
# use devela::{ansi, const_assert};
assert_eq![&[27, 91, 49, 109], ansi![b: bold]];
assert_eq![&[27, 91, 49, 109, 27, 91, 51, 109], ansi![b: bold, ITALIC]];
assert_eq![&[27, 91, 50, 59, 51, 72], ansi![b: cursor_move1(2, 3)]];
const_assert![eq_buf
    &[27, 91, 49, 109, 27, 91, 51, 51, 109, 27, 91, 52, 59, 50, 72, 27, 91, 48, 109],
    ansi![b: bold yElLoW, cursor_move1(4, 2) rEsEt]
];

assert_eq!["\u{1b}[1m", ansi![s: bold]];
assert_eq!["\u{1b}[1m\u{1b}[3m", ansi![s: bold, ITALIC]];
assert_eq!["\u{1b}[2;3H", ansi![s: cursor_move1(2, 3)]];
const_assert![eq_str
    "\u{1b}[1m\u{1b}[33m\u{1b}[4;2H\u{1b}[0m",
    ansi![s: bold yElLoW, cursor_move1(4, 2) rEsEt]
];

# #[cfg(feature = "std")] {
// prints the codes to `stdout`
ansi![p: bold, ITALIC, cursor_move1(2, 3)].unwrap();
# }
```
[0]: super::Ansi#ansi-escape-codes"#;
}

// non-printing version fallback (not(std), not(linux))
// -----------------------------------------------------------------------------
#[doc = DOC_ANSI!()]
#[cfg(not(any(
    /* 1) */ feature = "std",
    /* 2) */ all(feature = "linux", feature = "unsafe_syscall", not(miri),
        any(
            target_arch = "x86", target_arch = "x86_64",
            target_arch = "arm", target_arch = "aarch64",
            target_arch = "riscv32", target_arch = "riscv64"
        ),
))))]
#[doc(hidden)]
#[macro_export]
macro_rules! ansi {
    (
    b: // outputs a static byte slice
    $($command:ident $(($($arg:expr),*))? $(,)?)+) => {{
        const BYTES: &'static [u8] = $crate::paste! {
            $crate::join!(bytes: $( $crate::Ansi::[<$command:upper>] $(($($arg),*))? ,)+ )
        };
        BYTES
    }};
    (
    s: // outputs a static string slice
    $($arg:tt)*) => { $crate::Str::__utf8_bytes_to_str($crate::ansi![b: $($arg)*]) };
    (
    p: // no-op (printing not supported)
    $($arg:tt)*) => {};
    (
    @p: // no-op (printing not supported)
    $($command:ident $(($($arg:expr),*))? $(,)?)+) => {};
}

// std version (not(linux))
// -----------------------------------------------------------------------------
#[doc = DOC_ANSI!()]
#[cfg(feature = "std")]
#[cfg(not(all(feature = "linux", feature = "unsafe_syscall", not(miri),
    any( // targets:
        target_arch = "x86", target_arch = "x86_64",
        target_arch = "arm", target_arch = "aarch64",
        target_arch = "riscv32", target_arch = "riscv64"
    ),
)))]
#[doc(hidden)]
#[macro_export]
macro_rules! ansi {
    (
    b: // outputs a static byte slice
    $($command:ident $(($($arg:expr),*))? $(,)?)+) => {{
        const BYTES: &'static [u8] = $crate::paste! {
            $crate::join!(bytes: $( $crate::Ansi::[<$command:upper>] $(($($arg),*))? ,)+ )
        };
        BYTES
    }};
    (
    s: // outputs a static string slice
    $($arg:tt)*) => { $crate::Str::__utf8_bytes_to_str($crate::ansi![b: $($arg)*]) };
    (
    p: // print static commands (prints all commands concatenated)
    $($arg:tt)*) => { $crate::Ansi::print_std( $crate::ansi![b: $($arg)*] ) };
    (
    @p: // print dynamic commands (prints each command immediately)
    $($command:ident $(($($arg:expr),*))? $(,)?)+) => {{
        let mut some_error = None;
        $(
            if some_error.is_none() {
                match $crate::Ansi::print_std( $crate::paste! {
                    &$crate::Ansi::[<$command:upper>] $(($($arg),*))?
                }) {
                    Ok(_) => (),
                    Err(e) => some_error = Some(e),
                }
            }
        )+
        if let Some(e) = some_error { Err(e) } else { Ok(()) }
    }};
}

// linux version (overrides std)
// -----------------------------------------------------------------------------
#[doc = DOC_ANSI!()]
#[cfg(all(
    feature = "linux", feature = "unsafe_syscall", not(miri),
    any( // targets:
        target_arch = "x86", target_arch = "x86_64",
        target_arch = "arm", target_arch = "aarch64",
        target_arch = "riscv32", target_arch = "riscv64"
    ),
))]
#[doc(hidden)]
#[macro_export]
macro_rules! ansi {
    (
    b: // outputs a static byte slice
    $($command:ident $(($($arg:expr),*))? $(,)?)+) => {{
        const BYTES: &'static [u8] = $crate::paste! {
            $crate::join!(bytes: $( $crate::Ansi::[<$command:upper>] $(($($arg),*))? ,)+ )
        };
        BYTES
    }};
    (
    s: // outputs a static string slice
    $($arg:tt)*) => { $crate::Str::__utf8_bytes_to_str($crate::ansi![b: $($arg)*]) };
    (
    p: // print static commands (prints all commands concatenated)
    $($arg:tt)*) => { $crate::Ansi::print_linux( $crate::ansi![b: $($arg)*] ) };
    (
    @p: // print dynamic commands (prints each command immediately)
    $($command:ident $(($($arg:expr),*))? $(,)?)+) => {{
        let mut some_error = None;
        $(
            if some_error.is_none() {
                match $crate::Ansi::print_linux( $crate::paste! {
                    &$crate::Ansi::[<$command:upper>] $(($($arg),*))?
                }) {
                    Ok(_) => (),
                    Err(e) => some_error = Some(e),
                }
            }
        )+
        if let Some(e) = some_error { Err(e) } else { Ok(()) }
    }};
}

#[doc(inline)]
pub use ansi;

// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use crate::{ansi, const_assert};

    #[test]
    fn str() {
        assert_eq!["\u{1b}[1m", ansi![s: bold]];
        assert_eq!["\u{1b}[1m\u{1b}[3m", ansi![s: bold, ITALIC]];
        assert_eq!["\u{1b}[2;3H", ansi![s: cursor_move1(2, 3)]];

        const_assert![eq_str "\u{1b}[1m", ansi![s: bold]];
        const_assert![eq_str "\u{1b}[1m\u{1b}[3m", ansi![s: bold, ITALIC]];
        const_assert![eq_str "\u{1b}[2;3H", ansi![s: cursor_move1(2, 3)]];
    }

    #[test]
    fn bytes() {
        assert_eq![&[27, 91, 49, 109], ansi![b: bold]];
        assert_eq![&[27, 91, 49, 109, 27, 91, 51, 109], ansi![b: bold, ITALIC]];
        assert_eq![&[27, 91, 50, 59, 51, 72], ansi![b: cursor_MOve1(2, 3)]];

        const_assert![eq_buf & [27, 91, 49, 109], ansi![b: bold]];
        const_assert![eq_buf & [27, 91, 49, 109, 27, 91, 51, 109], ansi![b: bold, ITALIC]];
        const_assert![eq_buf & [27, 91, 50, 59, 51, 72], ansi![b: cursor_MOve1(2, 3)]];
    }

    #[test]
    #[cfg(any(
        /* 1) */ feature = "std",
        /* 2) */ all(feature = "linux", feature = "unsafe_syscall", not(miri),
            any(
                target_arch = "x86", target_arch = "x86_64",
                target_arch = "arm", target_arch = "aarch64",
                target_arch = "riscv32", target_arch = "riscv64"
            ),
    )))]
    fn print_non_const() {
        fn dyn_args(x: u8, y: u8) {
            let _ = ansi![@p: cursor_save cursor_move1(x, y) cursor_restore];
        }
        dyn_args(3, 5);
        dyn_args(4, 8);
    }
}
