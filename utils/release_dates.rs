#!/usr/bin/env -S rust-script -c
//! ```cargo
//! [dependencies]
//! jiff = "0.1"
//! ```
//
// Links for next releases:
// - https://releases.rs/
// - https://github.com/rust-lang/rust/milestones
// - https://doc.rust-lang.org/edition-guide/rust-2024/index.html
// - https://calendar.google.com/calendar/u/0/embed?src=mozilla.com_ts4qudb88i0tbjef8rche4a6v4@group.calendar.google.com

use jiff::{civil::Date, ToSpan, Zoned};
use std::process::Command;

/// The date of release of Rust 1.1.
const RUST_1_1: Date = Date::constant(2015, 6, 25);

/// Returns the date of release of the `nth` rust version.
//
// Rust 1.1 release was 6 weeks and 1 day after 1.0 (2015-05-15),
// but after version 1.1 it's always 6 weeks in between releases.
// (generally at 16:00 UTC)
#[inline]
fn nth_rust_version_date(nth: u32) -> Date {
    RUST_1_1 + 6.weeks() * (nth - 1) as i64
}

/// Calculates the current stable Rust version based on the current date.
///
/// Returns the medium version number (e.g., for Rust 1.82.0, returns 82).
fn rust_version_from_date(date: Date) -> u32 {
    let duration = RUST_1_1.until(date).unwrap();
    let total_days = duration.get_days();
    // Each Rust release occurs every 6 weeks (42 days)
    let n = (total_days / 42) + 1;
    n as u32
}

/// Returns the medium number of the current detected Rust version.
///
/// If `rustc` is not found, computes the version based on the current date.
fn current_rust_version() -> u32 {
    if let Ok(output) = Command::new("rustc").arg("--version").output() {
        if let Ok(version_str) = String::from_utf8(output.stdout) {
            if let Some(version_part) = version_str.split_whitespace().nth(1) {
                eprintln!("Current detected Rust version: {version_part}");
                if let Some(medium_version) = version_part.split('.').nth(1) {
                    if let Ok(version) = medium_version.parse() {
                        return version;
                    }
                }
            }
        }
    }
    // If rustc is not found or parsing fails, compute version from date
    let today = Zoned::now().date();
    let computed_version = rust_version_from_date(today);
    eprintln!("Computed current Rust version: 1.{computed_version}");
    computed_version
}

/// Prints the previous `p` releases (including the current one) and the next `n`.
fn print_prev_next_n_releases(p: u32, n: u32) {
    let today = Zoned::now().date();
    let current = current_rust_version();
    for n in current - p + 1..current + n + 1 {
        let date = nth_rust_version_date(n);
        let remain = today.until(date).unwrap();
        // WAIT: [fmt::friendly](https://github.com/BurntSushi/jiff/issues/111)
        println!("1.{n}_{date}, ({remain}) ");
    }
}

fn main() {
    print_prev_next_n_releases(3, 3);
    println!();
}
