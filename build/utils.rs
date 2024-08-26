// devela build::utils
//
//! build script utility functions
//

#![allow(dead_code)]

use std::{env, path::PathBuf};

/// Retuns the path of `OUT_DIR`.
pub(crate) fn out_dir_path() -> PathBuf {
    PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR not set"))
}

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

/// Prints the build script `start` or end message to *stdout*.
#[cfg(feature = "__dbg")]
pub(crate) fn println_start_end(start: bool) {
    let msg = if start {
        "~ Start of build script ~"
    } else {
        println("");
        "~ End of build script ~"
    };
    let line = "~".repeat(msg.len());
    println(&line);
    println(msg);
    println(&line);
}
