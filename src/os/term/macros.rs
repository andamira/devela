// devela::os::term::ansi::macros
//
//! ANSI macros.
//

/// Concatenates [`Ansi` escape codes][0], and returns a [`&[u8]`](slice).
///
/// - codes are case-insensitive.
/// - accepts a list of codes separated by spaces and/or commas.
/// - codes without parenthesis refers to `Ansi` associated constants.
/// - codes identifiers with parenthesis refers to `Ansi` associated functions.
///
/// # Examples
/// ```
/// use devela::os::term::ansib;
///
/// assert_eq![&[27, 91, 49, 109], ansib![bold]];
/// assert_eq![&[27, 91, 49, 109, 27, 91, 51, 109], ansib![bold, ITALIC]];
/// assert_eq![&[27, 91, 50, 59, 51, 72], ansib![cursor_move1(2, 3)]];
///
/// const CODE: &[u8] = ansib![bold italic cursor_up2(23)];
///
/// assert_eq![
///     &[27, 91, 49, 109, 27, 91, 51, 51, 109, 27, 91, 52, 59, 50, 72, 27, 91, 48, 109],
///     ansib![bold yElLoW, cursor_move1(4, 2) rEsEt]
/// ];
/// ```
///
/// See [`ansi`] for the `&str` version and [`ansip`] for the printing version.
///
/// [0]: super::Ansi#ansi-escape-codes
#[macro_export]
#[cfg_attr(feature = "nightly", doc(cfg(all(feature = "dep", feature = "text"))))]
#[cfg(any(all(feature = "dep", feature = "text"), feature = "const-str"))]
macro_rules! ansib {
    ( $( $command:ident $( ( $($arg:expr),* ) )? $(,)? )+ ) => { $crate::meta::paste! {
        $crate::_dep::const_str::concat_bytes!(
            $($crate::os::term::Ansi::[<$command:upper>] $( ($($arg),*) )? ,)+
        )
    }};
}
#[doc(inline)]
#[cfg(any(all(feature = "dep", feature = "text"), feature = "const-str"))]
pub use ansib;

/// Concatenates [`Ansi` escape codes][0], and returns a [`&str`].
///
/// - codes are case-insensitive.
/// - accepts a list of codes separated by spaces and/or commas.
/// - codes without parenthesis refers to `Ansi` associated constants.
/// - codes identifiers with parenthesis refers to `Ansi` associated functions.
///
/// # Examples
/// ```
/// use devela::os::term::ansi;
///
/// assert_eq!["\u{1b}[1m", ansi![bold]];
/// assert_eq!["\u{1b}[1m\u{1b}[3m", ansi![bold, ITALIC]];
/// assert_eq!["\u{1b}[2;3H", ansi![cursor_move1(2, 3)]];
///
/// const CODE: &str = ansi![bold italic cursor_up2(23)];
///
/// assert_eq![
///     "\u{1b}[1m\u{1b}[33m\u{1b}[4;2H\u{1b}[0m",
///     ansi![bold yElLoW, cursor_move1(4, 2) rEsEt]
/// ];
/// ```
///
/// See [`ansib`] for the `&[u8]` version and [`ansip`] for the printing version.
///
/// # Features
/// The `unsafe_str` feature allows the unchecked conversion to `&str`
/// of the already valid ASCII encoding from [`ansib`].
///
/// [0]: super::Ansi#ansi-escape-codes
#[macro_export]
#[cfg_attr(feature = "nightly", doc(cfg(all(feature = "dep", feature = "text"))))]
#[cfg(any(all(feature = "dep", feature = "text"), feature = "const-str"))]
macro_rules! ansi {
    ($($arg:tt)*) => {
        if cfg!(feature = "unsafe_str") {
            // SAFETY: ANSI escape codes are always ASCII and therefore utf-8 compatible
            unsafe {
                core::str::from_utf8_unchecked($crate::os::term::ansib![ $($arg)* ])
            }
        } else {
            if let Ok(s) = core::str::from_utf8($crate::os::term::ansib![ $($arg)* ]) {
                s
            } else {
                unreachable![]
            }
        }
    };
}
#[doc(inline)]
#[cfg(any(all(feature = "dep", feature = "text"), feature = "const-str"))]
pub use ansi;

/// Prints concatenated [`Ansi` escape codes][0]
/// using the Ansi [`print`][super::Ansi#method.print] method.
///
/// - codes are case-insensitive.
/// - accepts a list of codes separated by spaces and/or commas.
/// - codes without parenthesis refers to `Ansi` associated constants.
/// - codes identifiers with parenthesis refers to `Ansi` associated functions.
///
/// # Examples
/// ```
/// use devela::os::term::ansip;
///
/// // prints the codes to `stdout`
/// ansip![bold, ITALIC, cursor_move1(2, 3)];
/// ```
///
/// See [`ansi!`] and [`ansib`] for the non-printing versions.
///
/// [0]: super::Ansi#ansi-escape-codes
#[cfg_attr(
    feature = "nightly",
    doc(cfg(all(
        feature = "dep",
        feature = "text",
        any(feature = "std", feature = "linux_unsafe"),
    )))
)]
#[cfg(all(
    not(miri),
    any(feature = "const-str", all(feature = "dep", feature = "text"),),
    any(
        feature = "std",
        all(
            feature = "linux",
            feature = "unsafe_linux",
            any(
                target_arch = "x86_64",
                target_arch = "x86",
                target_arch = "arm",
                target_arch = "aarch64",
                target_arch = "riscv32",
                target_arch = "riscv64"
            )
        )
    )
))]
#[macro_export]
macro_rules! ansip {
    ($($arg:tt)*) => {
        $crate::os::term::Ansi::print( $crate::os::term::ansib![ $($arg)* ] );
    };
}

#[cfg(all(
    not(miri),
    any(feature = "const-str", all(feature = "dep", feature = "text"),),
    any(
        feature = "std",
        all(
            feature = "linux",
            feature = "unsafe_linux",
            any(
                target_arch = "x86_64",
                target_arch = "x86",
                target_arch = "arm",
                target_arch = "aarch64",
                target_arch = "riscv32",
                target_arch = "riscv64"
            )
        )
    )
))]
#[doc(inline)]
pub use ansip;
