// devela::text::ext::string
//
//!
//

#[cfg(feature = "alloc")]
use crate::_dep::_liballoc::string::{String, ToString};
#[allow(unused_imports)] // IMPROVE: impl for ArrayString, …
use crate::text::AsciiChar;

// Marker trait to prevent downstream implementations of the `ExtString` trait.
#[cfg(feature = "alloc")]
impl private::Sealed for String {}
#[cfg(feature = "alloc")]
mod private {
    pub trait Sealed {}
}

/// Extension trait providing additional methods for [`String`].
#[cfg(feature = "alloc")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "alloc")))]
pub trait ExtString: private::Sealed {
    /// Returns a [`String`] where you always know each character's position.
    ///
    /// A [*counter string*][0] is a graduated string of arbitrary `length`,
    /// with a `separator` positioned after the immediately preceding number.
    ///
    /// # Examples
    /// ```
    /// use devela::text::{AsciiChar, ExtString};
    ///
    /// assert_eq!("2*4*6*8*11*14*", String::new_counter(14, AsciiChar::Asterisk));
    /// assert_eq!("_3_5_7_9_12_15_", String::new_counter(15, AsciiChar::LowLine));
    /// ```
    /// [0]: https://www.satisfice.com/blog/archives/22
    #[must_use]
    fn new_counter(length: usize, separator: AsciiChar) -> String;
}

#[cfg(feature = "alloc")]
impl ExtString for String {
    fn new_counter(mut length: usize, separator: AsciiChar) -> String {
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
}
