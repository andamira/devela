#!/usr/bin/env -S rust-script -c
//! ```cargo
//! [package]
//! name = "croot"
//! version = "0.1.0"
//! description = "Get the path to your Cargo project root"
//! [dependencies.devela]
//! features = ["std", "linux"]
//! path = "../.."
//! ```
// installation:
// cargo install --path . --bin croot -F std,linux
//
// IMPROVE: add usage, help

use devela::{Env, FsPath, Process, ProcessExt};

fn main() {
    let args: Vec<String> = Env::args().collect();

    // concatenate the optional first argument
    let relative_path = args.get(1).map_or("", |s| s.as_str());

    match FsPath::from_crate_root(relative_path) {
        Ok(path) => println!("{}", path.display()),
        Err(e) => {
            eprintln!("Error finding crate root: {}", e);
            Process::exit(1);
        }
    }
}
