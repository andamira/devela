#!/usr/bin/env -S rust-script -c
//! ```cargo
//! [package]
//! name = "check_ansi_macro"
//! [dependencies.devela]
//! path = "../../../../../.."
//!
//! [features]
//! default = ["ansi_linux"]
//! # default = ["ansi_std"]
//! # default = ["ansi_fallback"]
//!
//! ansi_linux = ["devela/term", "devela/linux"]
//! ansi_std = ["devela/term", "devela/std"]
//! ansi_fallback = ["devela/term"]
//! ```
//
// Manual backend-matrix check for the `ansi!` macro.
//
// This file is not imported by the crate. Edit the local `[features]`
// default above to check the linux, std, and fallback macro backends.
//

/* compile checks */
#[cfg(not(any(feature = "ansi_linux", feature = "ansi_std", feature = "ansi_fallback")))]
compile_error!("enable exactly one of: ansi_linux, ansi_std, ansi_fallback");
#[cfg(any(
    all(feature = "ansi_linux", feature = "ansi_std"),
    all(feature = "ansi_linux", feature = "ansi_fallback"),
    all(feature = "ansi_std", feature = "ansi_fallback"),
))]
compile_error!("enable only one of: ansi_linux, ansi_std, ansi_fallback");
#[cfg(all(feature = "ansi_linux", not(target_os = "linux")))]
compile_error!("ansi_linux requires a Linux target");

use devela::ansi;

#[rustfmt::skip]
const BACKEND: &str = {
    #[cfg(feature = "ansi_linux")]
    { "linux" }
    #[cfg(feature = "ansi_std")]
    { "std" }
    #[cfg(feature = "ansi_fallback")]
    { "fallback" }
};

fn main() {
    println!("ansi! backend: {BACKEND}");

    check_bytes();
    check_str();
    check_static_print();
    check_dynamic_print();

    #[cfg(feature = "ansi_fallback")]
    check_fallback_does_not_eval_dynamic_args();

    println!("ansi! check ok");
}

fn check_bytes() {
    assert_eq!(&[27, 91, 49, 109], ansi![b: bold]);
    assert_eq!(&[27, 91, 49, 109, 27, 91, 51, 109], ansi![b: bold, ITALIC]);
    assert_eq!(&[27, 91, 50, 59, 51, 72], ansi![b: cursor_move1(3, 2)]);
    assert_eq!(
        &[
            27, 91, 49, 109, // bold
            27, 91, 51, 51, 109, // yellow
            27, 91, 52, 59, 50, 72, // cursor_move1(2, 4)
            27, 91, 48, 109, // reset
        ],
        ansi![b: bold yElLoW, cursor_move1(2, 4) rEsEt],
    );
}
fn check_str() {
    assert_eq!("\u{1b}[1m", ansi![s: bold]);
    assert_eq!("\u{1b}[1m\u{1b}[3m", ansi![s: bold, ITALIC]);
    assert_eq!("\u{1b}[2;3H", ansi![s: cursor_move1(3, 2)]);

    assert_eq!(
        "\u{1b}[1m\u{1b}[33m\u{1b}[4;2H\u{1b}[0m",
        ansi![s: bold yElLoW, cursor_move1(2, 4) rEsEt],
    );
}

fn check_static_print() {
    // In linux/std this prints.
    // In fallback this validates the command sequence but does not print.
    ansi![p: reset].expect("ansi![p:] failed");
    ansi![p: bold, reset].expect("ansi![p:] failed");

    // unwrap arm
    ansi![p! reset];
}

fn check_dynamic_print() {
    let row = 20;
    let col = 30;

    // In linux/std this prints each command immediately.
    // In fallback this validates names/types but does not execute arguments.
    ansi![@p: cursor_move2(col, row), reset].expect("ansi![@p:] failed");

    // unwrap arm
    ansi![@p! reset];
}

#[cfg(feature = "ansi_fallback")]
fn check_fallback_does_not_eval_dynamic_args() {
    use devela::{AtomicOrdering::SeqCst, AtomicUsize};

    static CALLS: AtomicUsize = AtomicUsize::new(0);

    fn next_cell() -> u8 {
        CALLS.fetch_add(1, SeqCst);
        1
    }
    ansi![@p: cursor_move2(next_cell(), next_cell()), reset]
        .expect("fallback @p validation failed");
    assert_eq!(
        0,
        CALLS.load(SeqCst),
        "fallback @p must type-check dynamic arguments without evaluating them",
    );
}
