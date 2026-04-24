#!/usr/bin/env -S rust-script -c --debug
//! ```cargo
//! [dependencies]
//! devela = { path = "../../../..", features = ["std", "_docs_examples"]}
//! ```
// WAIT:[cargo-script](https://github.com/rust-lang/cargo/issues/12207)
//

use devela::cdbg;

fn section(name: &str) {
    eprintln!("\n--- {name} ---");
}

fn main() {
    let x = 7;
    let y = 11;
    let pair = ("hi", [1, 2, 3]);

    section("no-op");
    cdbg!();
    cdbg![;];

    section("default, no location");
    let _ = cdbg![x];
    let _ = cdbg![@ x];
    let _ = cdbg![# pair];
    let _ = cdbg![x, y];
    let _ = cdbg![# x, pair];

    section("custom prefix, no location");
    let _ = cdbg!["x: "; x];
    let _ = cdbg![""; x];
    let _ = cdbg![; x];
    let _ = cdbg!["pair:\n"; # pair];
    let _ = cdbg!["p: "; x, y];
    let _ = cdbg![; x, y];
    let _ = cdbg!["P: "; # x, pair];

    section("numeric location");
    let _ = cdbg![0@ x];
    let _ = cdbg![1@ x];
    let _ = cdbg![2@ x];
    let _ = cdbg![1@ x, y];
    cdbg![1@];

    section("numeric location, pretty");
    let _ = cdbg![0# pair];
    let _ = cdbg![1# pair];
    let _ = cdbg![2# pair];
    let _ = cdbg![1# x, pair];
    cdbg![1#];

    section("numeric location with custom prefix");
    let _ = cdbg!["x: "; 1@ x];
    let _ = cdbg![; 1@ x];
    let _ = cdbg!["pair:\n"; 1# pair];
    let _ = cdbg![; 1# pair];

    section("full path location");
    let _ = cdbg![f@ x];
    let _ = cdbg![f @ x, y];
    cdbg![f @];

    section("full path location, pretty");
    let _ = cdbg![f # pair];
    let _ = cdbg![f # x, pair];
    cdbg![f #];

    section("full path location with custom prefix");
    let _ = cdbg!["x: "; f@ x];
    let _ = cdbg![; f@ x];
    let _ = cdbg!["pair:\n"; f # pair];
    let _ = cdbg![; f # pair];

    section("full path location on separate line");
    let _ = cdbg![fln@ x];
    let _ = cdbg![fln @ x, y];
    cdbg![fln @];

    section("full path location on separate line, pretty");
    let _ = cdbg![fln # pair];
    let _ = cdbg![fln # x, pair];
    cdbg![fln #];

    section("full path location on separate line with custom prefix");
    let _ = cdbg!["x: "; fln@ x];
    let _ = cdbg![; fln@ x];
    let _ = cdbg!["pair:\n"; fln # pair];
    let _ = cdbg![; fln # pair];
}
