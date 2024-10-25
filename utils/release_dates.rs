#!/usr/bin/env -S rust-script -c
//! ```cargo
//! [dependencies]
//! jiff = "0.1"
//! ```
//

use jiff::{civil::Date, ToSpan, Zoned};
use std::process::Command;

/// Returns the date of release of the `nth` rust version.
fn nth_rust_version_date(nth: u32) -> Date {
    // The release date of Rust 1.1 was 6 weeks and 1 day after 1.0 (2015-05-15).
    // After version 1.1 it's going to always be 6 weeks between releases.
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
