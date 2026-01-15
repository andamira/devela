// devela::text::ext::string
//
//!
//

#[cfg(feature = "alloc")]
use crate::{Arc, Box, Rc, Str, String, ToString, Vec, vec_};

/// Marker trait to prevent downstream implementations of the [`StringExt`] trait.
#[cfg(feature = "alloc")]
trait Sealed {}
#[cfg(feature = "alloc")]
impl Sealed for String {}

#[doc = crate::_tags!(text)]
/// Extension trait providing additional methods for [`String`].
#[expect(private_bounds, reason = "Sealed")]
#[cfg(feature = "alloc")]
#[cfg_attr(nightly_doc, doc(notable_trait, cfg(feature = "alloc")))]
pub trait StringExt: Sealed {
    /// Converts the string into a `Box<str>`.
    ///
    /// Allows single ownership with exact allocation,
    /// for when you don't need to clone or share.
    fn to_box(self) -> Box<str>;

    /// Converts the string into an `Rc<str>`.
    ///
    /// Allows shared ownership with reference counting,
    /// reducing memory duplication in single-threaded scenarios.
    fn to_rc(self) -> Rc<str>;

    /// Converts the string into an `Arc<str>`.
    ///
    /// When you need shared ownership of a string slice across multiple threads.
    fn to_arc(self) -> Arc<str>;

    /// Returns a [`String`] where you always know each character's position.
    ///
    /// A [*counter string*][0] is a graduated string of arbitrary `length`,
    /// with a `separator` positioned after the immediately preceding number.
    ///
    /// # Examples
    /// ```
    /// use devela::StringExt;
    ///
    /// assert_eq!("2*4*6*8*11*14*", String::new_counter(14, '*'));
    /// assert_eq!("_3_5_7_9_12_15_", String::new_counter(15, '_'));
    /// ```
    /// # Panics
    /// Panics if `!separator.is_ascii()`.
    ///
    /// # Features
    /// Makes use of the `unsafe_str` feature if enabled.
    ///
    /// [0]: https://www.satisfice.com/blog/archives/22
    #[must_use]
    fn new_counter(length: usize, separator: char) -> String;
}

#[cfg(feature = "alloc")]
impl StringExt for String {
    /// It just calls the method [`String::into_boxed_str`].
    fn to_box(self) -> Box<str> {
        self.into_boxed_str()
    }
    fn to_rc(self) -> Rc<str> {
        Rc::from(self)
    }
    fn to_arc(self) -> Arc<str> {
        Arc::from(self)
    }

    fn new_counter(mut length: usize, separator: char) -> String {
        #[cfg(any(feature = "safe_text", not(feature = "unsafe_str")))]
        {
            let mut buf = vec_![0u8; length];
            let s = Str::new_counter(&mut buf, length, separator);
            let len = s.len();
            buf.truncate(len);
            String::from_utf8(buf).expect("counter string is guaranteed ASCII")
        }
        #[cfg(all(not(feature = "safe_text"), feature = "unsafe_str"))]
        {
            let mut s = String::with_capacity(length);
            // SAFETY: only ASCII bytes are written and the str.len() is set to the written prefix
            unsafe {
                let buf = s.as_mut_vec();
                buf.resize(length, 0);
                let out = Str::new_counter(buf, length, separator);
                let len = out.len();
                buf.truncate(len);
            }
            s
        }
    }
}
