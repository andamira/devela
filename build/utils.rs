// devela build::utils
//
//! build script utility functions
//

#![allow(dead_code)]

use std::{env, path::PathBuf};

/// Prints a message to *stdout* from the build script.
#[cfg(feature = "__dbg")]
pub(crate) fn println(msg: &str) {
    println!("cargo:warning={}", msg);
}

/// Prints a heading message to *stdout*, underlined.
#[cfg(feature = "__dbg")]
pub(crate) fn println_heading(msg: &str) {
    println("");
    println(msg);
    println(&"-".repeat(msg.len()));
}

/// Prints the value of an environment variable.
#[cfg(feature = "__dbg")]
pub(crate) fn println_var(var: &str) {
    if let Ok(v) = env::var(var) {
        println(&format!["· {var}: {v}"]);
    } else {
        // println(&format!["Environment variable `{var}` not set"]);
        println(&format!["x {var}:"]);
    }
}

/// Retuns the path of `OUT_DIR`.
pub(crate) fn out_dir_path() -> PathBuf {
    PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR not set"))
}
