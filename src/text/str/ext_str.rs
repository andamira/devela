// devela::text::ext::slice
//
//! Defines the [`StrExt`] trait.
//
// WAIT: [str_as_str](https://github.com/rust-lang/rust/issues/130366)
// WAIT: [substr_range](https://github.com/rust-lang/rust/issues/126769)

use crate::Str;
#[cfg(feature = "alloc")]
use crate::{Arc, Box, Rc};
crate::_use! {basic::from_utf8}

/// Marker trait to prevent downstream implementations of the [`StrExt`] trait.
trait Sealed {}
impl Sealed for str {}

#[doc = crate::_tags!(text)]
/// Extension trait providing additional methods for [`&str`].
#[cfg_attr(nightly_doc, doc(notable_trait))]
#[expect(private_bounds, reason = "Sealed")]
pub trait StrExt: Sealed {
    /// Converts the string slice into a `Box<str>`.
    ///
    /// Allows single ownership with exact allocation,
    /// for when you don't need to clone or share.
    #[cfg(feature = "alloc")]
    fn to_box(&self) -> Box<str>;

    /// Converts the string slice into an `Rc<str>`.
    ///
    /// Allows shared ownership with reference counting,
    /// reducing memory duplication in single-threaded scenarios.
    #[cfg(feature = "alloc")]
    fn to_rc(&self) -> Rc<str>;

    /// Converts the string slice into an `Arc<str>`.
    ///
    /// When you need shared ownership of a string slice across multiple threads.
    #[cfg(feature = "alloc")]
    fn to_arc(&self) -> Arc<str>;

    /// Repeats a string a given number of times into the provided `buffer`.
    /// and returns a reference to the new `&str`.
    /// # Examples
    /// ```
    /// use devela::StrExt;
    ///
    /// let mut buf = [0_u8; 12];
    /// let repeated = "ay".repeat_into(3, &mut buf);
    /// assert_eq![repeated, "ayayay"];
    /// ```
    /// # Features
    /// Makes use of the `unsafe_str` feature if enabled.
    ///
    /// For the *const* version see [`Str::repeat_into`].
    #[must_use]
    fn repeat_into<'input, const CAP: usize>(
        &self,
        n: usize,
        buffer: &'input mut [u8; CAP],
    ) -> &'input str;

    /// Repeats `string` up to `n` times into `buffer`,
    /// stopping early if it does not fit,
    /// and returns the written prefix as `&str`.
    ///
    /// Like [`repeat_into`], but accepts a dynamically sized buffer.
    fn repeat_into_slice<'input, const CAP: usize>(
        &self,
        n: usize,
        buffer: &'input mut [u8],
    ) -> &'input str;

    /// Returns a [`&str`] backed by a `buffer`, where you always know each
    /// character's position.
    ///
    /// A [*counter string*][0] is a graduated string of arbitrary `length`,
    /// with a `separator` positioned after the immediately preceding number.
    /// # Examples
    /// ```
    /// use devela::StrExt;
    ///
    /// let mut buf = [0; 15];
    /// assert_eq!("2*4*6*8*11*14*", str::new_counter(&mut buf, 14, '*'));
    /// assert_eq!("_3_5_7_9_12_15_", str::new_counter(&mut buf, 15, '_'));
    /// ```
    /// # Panics
    /// Panics if `buffer.len() < length`, or if `!separator.is_ascii()`.
    ///
    /// # Features
    /// Makes use of the `unsafe_str` feature if enabled.
    ///
    /// For the *const* version see [`Str::new_counter`].
    ///
    /// [0]: https://www.satisfice.com/blog/archives/22
    #[must_use]
    fn new_counter(buffer: &mut [u8], length: usize, separator: char) -> &str;
}

impl StrExt for str {
    #[cfg(feature = "alloc")]
    fn to_box(&self) -> Box<str> {
        Box::from(self)
    }
    #[cfg(feature = "alloc")]
    fn to_rc(&self) -> Rc<str> {
        Rc::from(self)
    }
    #[cfg(feature = "alloc")]
    fn to_arc(&self) -> Arc<str> {
        Arc::from(self)
    }

    fn repeat_into<'input, const CAP: usize>(
        &self,
        n: usize,
        buffer: &'input mut [u8; CAP],
    ) -> &'input str {
        Str::repeat_into(self, n, buffer)
    }
    fn repeat_into_slice<'input, const CAP: usize>(
        &self,
        n: usize,
        buffer: &'input mut [u8],
    ) -> &'input str {
        Str::repeat_into_slice(self, n, buffer)
    }

    fn new_counter(buffer: &mut [u8], length: usize, separator: char) -> &str {
        Str::new_counter(buffer, length, separator)
    }
}
