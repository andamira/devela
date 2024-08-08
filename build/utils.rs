// devela build::utils
//
//! build script utility functions
//

#![allow(dead_code)]

use std::{env, path::PathBuf};

/// Convenience function for printing messages from the build script.
#[cfg(feature = "__dbg")]
pub(crate) fn println(msg: &str) {
    println!("cargo:warning={}", msg);
}

pub(crate) fn out_dir() -> String {
    env::var("OUT_DIR").expect("OUT_DIR not set")
}

pub(crate) fn out_dir_path() -> PathBuf {
    PathBuf::from(out_dir())
}
