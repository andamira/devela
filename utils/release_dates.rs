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

/// Returns the date of release of the `nth` rust version.
//
// Rust 1.1 release was 6 weeks and 1 day after 1.0 (2015-05-15),
// but after version 1.1 it's always 6 weeks in between releases.
#[inline]
fn nth_rust_version_date(nth: u32) -> Date {
    Date::constant(2015, 6, 25) + 6.weeks() * (nth - 1) as i64
}

/// Returns the medium number of the current detected rust version.
fn current_rust_version() -> Option<u32> {
    let output = Command::new("rustc").arg("--version").output().ok()?;
    let version_str = String::from_utf8(output.stdout).ok()?;
    let version_part = version_str.split_whitespace().nth(1)?;
    eprintln!("Current detected rust version: {version_part}");
    let medium_version = version_part.split('.').nth(1)?;
    medium_version.parse().ok()
}

/// Prints the previous `p` releases (including the current one) and the next `n`.
fn print_prev_next_n_releases(p: u32, n: u32) {
    let today = Zoned::now().date();
    let current = if let Some(current) = current_rust_version() { current } else { 81 };
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
