// devela
//
//! Rust development helper & extension utilities.
//

#![warn(clippy::all)]
#![allow(
    clippy::float_arithmetic,
    clippy::implicit_return,
    clippy::needless_return,
    clippy::blanket_clippy_restriction_lints,
    clippy::pattern_type_mismatch
)]
//
#![cfg_attr(feature = "safe", forbid(unsafe_code))]
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(feature = "nightly", feature(doc_cfg))]

mod apply;
mod ops;
mod slice;
mod sugar;

pub use crate::{
    apply::{Also, Apply},
    ops::{pclamp, pmax, pmin},
    slice::{subslice_left, subslice_middle, subslice_right},
};

#[cfg(feature = "std")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "std")))]
#[doc(inline)]
pub use crate::sugar::bx;

#[cfg(feature = "std")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "std")))]
pub use std_utils::*;

#[cfg(feature = "std")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "std")))]
mod std_utils {
    use std::{
        convert::AsRef,
        env, fs, io,
        path::{Path, PathBuf},
    };

    /// Returns a [`String`] where you always know each character's position.
    ///
    /// A [*counter string*][0] is a graduated string of arbitrary `length`,
    /// with a `separator` positioned after the immediately preceding number.
    ///
    /// ## Examples
    ///
    /// ```
    /// use devela::counter_string;
    ///
    /// assert_eq!("2*4*6*8*11*14*", counter_string(14, '*'));
    /// assert_eq!("_3_5_7_9_12_15_", counter_string(15, '_'));
    /// ```
    ///
    /// [0]: https://www.satisfice.com/blog/archives/22
    pub fn counter_string(mut length: usize, separator: char) -> String {
        let mut cstr = String::new();

        while length > 0 {
            let mut tmpstr = separator.to_string();
            tmpstr.push_str(&length.to_string().chars().rev().collect::<String>());

            if tmpstr.len() > length {
                tmpstr = tmpstr[..length].to_string();
            }

            cstr.push_str(&tmpstr);
            length -= tmpstr.len();
        }
        cstr.chars().rev().collect::<String>()
    }

    /// Returns an absolute [`PathBuf`], relative to the `crate`'s root.
    ///
    /// It determines the root by finding the first `Cargo.toml` file, from the
    /// current directory through its ancestors.
    ///
    /// # Errors
    /// Returns an error if it can't find any `Cargo.toml` file,
    /// or if it encounters an invalid path during the search process.
    ///
    /// # Examples
    /// ```
    /// use devela::crate_root;
    ///
    /// match crate_root("") {
    ///     Ok(p) => println!("Current crate root is {:?}", p),
    ///     Err(e) => println!("Error obtaining crate root {:?}", e)
    /// };
    /// ```
    pub fn crate_root<P: AsRef<Path>>(path: P) -> io::Result<PathBuf> {
        let current_path = env::current_dir()?;
        let mut root_path = current_path.clone();

        for p in current_path.as_path().ancestors() {
            let has_cargo = fs::read_dir(p)?.any(|p| p.unwrap().file_name() == *"Cargo.toml");
            if has_cargo {
                return Ok(root_path.join(path.as_ref()));
            } else {
                root_path.pop();
            }
        }
        Err(io::Error::new(
            io::ErrorKind::NotFound,
            "Ran out of places to find Cargo.toml",
        ))
    }

    /// Like [`crate_root`] but returns a [`String`].
    ///
    /// In case of an error the returned string will be empty.
    pub fn crate_root_string<P: AsRef<Path>>(path: P) -> String {
        crate_root(Path::new(path.as_ref())).map_or("".into(), |p| p.to_str().unwrap().to_owned())
    }
}
