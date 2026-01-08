// devela_base_std::build::namespace
//
//! Defines the [`Build`] namespace.
//
// TOC
// - directories
// - print
// - other

use std::{env, path::PathBuf};

#[doc = crate::_TAG_NAMESPACE!()] // NOTE: use tag directly to work from /build
/// Memory-related operations.
///
/// See also: [`MemExt`][crate::MemExt], [`MemAligned`][crate::MemAligned]
/// [`Ptr`][crate::Ptr], [`Slice`][crate::Slice].
#[derive(Debug)]
pub struct Build;

impl Build {
    /// Returns the current crate's name.
    pub fn crate_name() -> String {
        env::var("CARGO_CRATE_NAME").expect("CARGO_CRATE_NAME not set")
    }

    /* directories */

    /// Retuns the path of `OUT_DIR`.
    pub fn out_dir() -> PathBuf {
        PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR not set"))
    }

    /// Retuns the path of `CARGO_MANIFEST_DIR`.
    pub fn manifest_dir() -> PathBuf {
        PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set"))
    }

    /// Retuns the path of `CARGO_MANIFEST_PATH`.
    pub fn manifest_path() -> PathBuf {
        PathBuf::from(env::var("CARGO_MANIFEST_PATH").expect("CARGO_MANIFEST_PATH not set"))
    }

    /* print */

    /// Prints a message to *stdout* from the build script.
    pub fn println(msg: &str) {
        println!("cargo:warning={}", msg);
    }

    /// Prints a heading message to *stdout*, underlined.
    pub fn println_heading(msg: &str) {
        Self::println("");
        Self::println(msg);
        Self::println(&"-".repeat(msg.len()));
    }

    /// Prints the value of an environment variable.
    pub fn println_var(var: &str) {
        if let Ok(v) = env::var(var) {
            Self::println(&format!["· {var}: {v}"]);
        } else {
            Self::println(&format!["x {var}:"]);
        }
    }
    /// Prints the value of an encoded environment variable,
    /// replacing unit separator characters `'0x1f'` with spaces.
    ///
    /// It accepts a new name to show for the decoded variable.
    pub fn println_var_encoded(var: &str, new_var_name: &str) {
        if let Ok(ev) = env::var(var) {
            let v = ev.replace('\x1f', " ");
            Self::println(&format!["· {new_var_name}(*): {v}"]);
        } else {
            Self::println(&format!["x {new_var_name}:"]);
        }
    }

    /// Prints the build script `start` or end message to *stdout*.
    pub fn println_start_end(name: &str, start: bool) {
        let msg = if start {
            format!("~ Start of {name} ~")
        } else {
            Self::println("");
            format!("~ End of {name} ~")
        };
        let line = "~".repeat(msg.len());
        Self::println(&line);
        Self::println(&msg);
        Self::println(&line);
    }
}

// RETHINK IMPROVE
#[rustfmt::skip]
impl Build {
    /// `n` tabs, `n`*4 spaces.
    pub const fn tab(n: usize) -> &'static str {
        match n {
            0 => "",
            1 => "    ",
            2 => "        ",
            3 => "            ",
            4 => "                ",
            5 => "                    ",
            6 => "                        ",
            7 => "                            ",
            8 => "                                ",
            _ => "                                    ",
        }
    }
    #[doc = "0 tabs, 0 spaces."]  pub const TAB0: &str = "";
    #[doc = "1 tabs, 4 spaces."]  pub const TAB1: &str = "    ";
    #[doc = "2 tabs, 8 spaces."]  pub const TAB2: &str = "        ";
    #[doc = "3 tabs, 12 spaces."] pub const TAB3: &str = "            ";
    #[doc = "4 tabs, 16 spaces."] pub const TAB4: &str = "                ";
    #[doc = "5 tabs, 20 spaces."] pub const TAB5: &str = "                    ";
    #[doc = "6 tabs, 24 spaces."] pub const TAB6: &str = "                        ";
    #[doc = "7 tabs, 28 spaces."] pub const TAB7: &str = "                            ";
    #[doc = "8 tabs, 32 spaces."] pub const TAB8: &str = "                                ";
    #[doc = "9 tabs, 36 spaces."] pub const TAB9: &str = "                                    ";
}
