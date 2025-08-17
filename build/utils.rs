// devela::build::utils
//
//! build script utility functions
//
// TOC
// - directories
// - print
// - other

#![allow(dead_code)]

use std::{env, path::PathBuf};

/* directories */

/// Retuns the path of `OUT_DIR`.
pub(crate) fn out_dir() -> PathBuf {
    PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR not set"))
}

/// Retuns the path of `CARGO_MANIFEST_DIR`.
pub(crate) fn manifest_dir() -> PathBuf {
    PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set"))
}

/// Retuns the path of `CARGO_MANIFEST_PATH`.
pub(crate) fn manifest_path() -> PathBuf {
    PathBuf::from(env::var("CARGO_MANIFEST_PATH").expect("CARGO_MANIFEST_PATH not set"))
}

/* print */

/// Prints a message to *stdout* from the build script.
#[cfg(feature = "__dbg")]
#[cfg_attr(nightly_doc, doc(cfg(feature = "__dbg")))]
pub(crate) fn println(msg: &str) {
    println!("cargo:warning={}", msg);
}

/// Prints a heading message to *stdout*, underlined.
#[cfg(feature = "__dbg")]
#[cfg_attr(nightly_doc, doc(cfg(feature = "__dbg")))]
pub(crate) fn println_heading(msg: &str) {
    println("");
    println(msg);
    println(&"-".repeat(msg.len()));
}

/// Prints the value of an environment variable.
#[cfg(feature = "__dbg")]
#[cfg_attr(nightly_doc, doc(cfg(feature = "__dbg")))]
pub(crate) fn println_var(var: &str) {
    if let Ok(v) = env::var(var) {
        println(&format!["· {var}: {v}"]);
    } else {
        // println(&format!["Environment variable `{var}` not set"]);
        println(&format!["x {var}:"]);
    }
}
/// Prints the value of an encoded environment variable,
/// replacing unit separator characters `'0x1f'` with spaces.
///
/// It accepts a new name to show for the decoded variable.
#[cfg(feature = "__dbg")]
#[cfg_attr(nightly_doc, doc(cfg(feature = "__dbg")))]
pub(crate) fn println_var_encoded(var: &str, new_var_name: &str) {
    if let Ok(ev) = env::var(var) {
        let v = ev.replace('\x1f', " ");
        println(&format!["· {new_var_name}(*): {v}"]);
    } else {
        // println(&format!["Environment variable `{var}` not set"]);
        println(&format!["x {new_var_name}:"]);
    }
}

/// Prints the build script `start` or end message to *stdout*.
#[cfg(feature = "__dbg")]
#[cfg_attr(nightly_doc, doc(cfg(feature = "__dbg")))]
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

/* other */

// skip formatting macro.
macro_rules! sf { ( $($line:tt)+ ) => { $($line)+ }; }

sf! {
#[doc = "0 tabs, 0 spaces."]  pub(crate) const TAB0: &str = "";
#[doc = "1 tabs, 4 spaces."]  pub(crate) const TAB1: &str = "    ";
#[doc = "2 tabs, 8 spaces."]  pub(crate) const TAB2: &str = "        ";
#[doc = "3 tabs, 12 spaces."] pub(crate) const TAB3: &str = "            ";
#[doc = "4 tabs, 16 spaces."] pub(crate) const TAB4: &str = "                ";
#[doc = "5 tabs, 20 spaces."] pub(crate) const TAB5: &str = "                    ";
#[doc = "6 tabs, 24 spaces."] pub(crate) const TAB6: &str = "                        ";
#[doc = "7 tabs, 28 spaces."] pub(crate) const TAB7: &str = "                            ";
#[doc = "8 tabs, 32 spaces."] pub(crate) const TAB8: &str = "                                ";
#[doc = "9 tabs, 36 spaces."] pub(crate) const TAB9: &str = "                                    ";
}
