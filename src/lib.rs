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
pub use apply::{Also, Apply};

/// Returns the minimum of two [`PartialOrd`]ered values.
///
/// Complements `core::cmp::`[`min`][`core::cmp::min] which requires
/// [`Ord`][core::cmp::Ord].
///
/// # Example
/// ```
/// use devela::pmin;
///
/// assert_eq![0.2, pmin(0.2, 0.4)];
/// ```
#[inline(always)]
#[rustfmt::skip]
pub fn pmin<T: PartialOrd>(a: T, b: T) -> T { if a < b { a } else { b } }

/// Returns the maximum of two [`PartialOrd`]ered values.
///
/// Complements `core::cmp::`[`max`][`core::cmp::max] which requires
/// [`Ord`][core::cmp::Ord].
///
/// # Examples
/// ```
/// use devela::pmax;
///
/// assert_eq![0.4, pmax(0.2, 0.4)];
/// ```
#[inline(always)]
#[rustfmt::skip]
pub fn pmax<T: PartialOrd>(a: T, b: T) -> T { if a > b { a } else { b } }

/// Returns a [`PartialOrd`]ered `value` clamped between `min` and `max`.
///
/// # Examples
/// ```
/// use devela::pclamp;
///
/// assert_eq![0.4, pclamp(1.0, 0.2, 0.4)];
/// assert_eq![0.2, pclamp(0.0, 0.2, 0.4)];
/// ```
#[inline(always)]
#[rustfmt::skip]
pub fn pclamp<T: PartialOrd>(value: T, min: T, max: T) -> T {
    pmin(pmax(value, min), max)
}

#[cfg(test)]
mod test_min_max_clamp {
    use super::{pclamp, pmax, pmin};

    #[test]
    fn min_max_clamp() {
        assert_eq![2, pmin(2, 5)];
        assert_eq![2, pmin(5, 2)];
        assert_eq![2., pmin(2., 5.)];

        assert_eq![5, pmax(2, 5)];
        assert_eq![5, pmax(5, 2)];
        assert_eq![5., pmax(2., 5.)];

        assert_eq![3, pclamp(3, 2, 5)];
        assert_eq![3., pclamp(3., 2., 5.)];
        assert_eq![2, pclamp(1, 2, 5)];
        assert_eq![5, pclamp(7, 2, 5)];
    }
}

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

    /// Brief [`Box`] constructor.
    ///
    /// # Examples
    /// ```
    /// use devela::bx;
    ///
    /// assert_eq![bx(45), Box::new(45)];
    /// ```
    #[inline(always)]
    pub fn bx<T>(v: T) -> Box<T> {
        Box::new(v)
    }

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

/// Returns a subslice at the left of `slice`, with max length `left_len`.
///
/// If `left_len > slice.len()` it returns the full slice.
pub fn subslice_left<T>(slice: &[T], left_len: usize) -> &[T] {
    let start_idx = 0;
    let end_idx = (left_len).clamp(start_idx, slice.len());
    &slice[start_idx..end_idx]
}

/// Returns a subslice from the right of `slice`, with max length `right_len`.
///
/// If `left_len > slice.len()` it returns the full slice.
pub fn subslice_right<T>(slice: &[T], right_len: usize) -> &[T] {
    let start_idx = slice.len().saturating_sub(right_len);
    let end_idx = slice.len();
    &slice[start_idx..end_idx]
}

/// Returns a subslice from the middle of `slice`, with max length `mid_len`.
///
/// If `mid_len > slice.len()` it returns the full slice.
///
/// In case of a non-perfect middle split, it leaves an extra left character.
pub fn subslice_mid<T>(slice: &[T], mid_len: usize) -> &[T] {
    let mid_idx = slice.len() / 2;
    let half_mid_len = mid_len / 2;
    let start_idx = mid_idx.saturating_sub(half_mid_len);
    let end_idx = (mid_idx + half_mid_len + (mid_len % 2)).min(slice.len());
    &slice[start_idx..end_idx]
}

/// *`i`nline `if`* macro.
///
/// # Examples
/// ```
/// use devela::iif;
///
/// let s = iif![1 > 0; true; false];
/// iif![1 > 0; println!("true")];
/// ```
/// instead of
/// ```
/// let s = if 1 > 0 {
///     true
/// } else {
///     false
/// };
/// if 1 > 0 {
///     println!("true");
/// }
/// ```
#[macro_export]
macro_rules! iif {
    ($if: expr; $true: expr) => {
        if $if {
            $true
        }
    };
    ($if: expr ; $true: expr ; $false: expr) => {
        if $if {
            $true
        } else {
            $false
        }
    };
}

#[cfg(test)]
mod test_iif {
    use crate::iif;

    #[test]
    fn iif() {
        assert_eq!('a', iif!(true ; 'a' ; 'b'));
        assert_eq!('b', iif!(false ; 'a' ; 'b'));
    }
}

/// *`r`ust `f`ormat `s`kip* macro.
///
/// Preserves the formatting of the code provided as arguments, by relying on
/// the fact that `rustfmt` does not usually apply formatting inside macros.
///
/// It can be used as an alternative to the `#[rustfmt::skip]` attribute,
/// specially where it can't be applied yet on stable rust.
///
/// # Examples
/// ```
/// use devela::rfs;
///
/// // rustfmt has no powers here
/// rfs! { println!(); for i in 0..3 { print!{"{i} "} } println!(); }
/// ```
#[macro_export]
macro_rules! rfs { ( $($line:tt)+ ) => { $($line)+ }; }
