// devela::string::ext::string
//
//!
//

#[cfg(feature = "alloc")]
use _alloc::string::String;

#[cfg(all(feature = "ascii", feature = "alloc"))]
use {crate::ascii::AsciiChar, _alloc::string::ToString};

// Marker trait to prevent downstream implementations of the `StringExt` trait.
#[cfg(feature = "alloc")]
impl private::Sealed for String {}
mod private {
    pub trait Sealed {}
}

/// Extension trait providing additional methods for owned strings.
#[cfg(feature = "alloc")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "alloc")))]
pub trait StringExt: private::Sealed {
    /// Returns a [`String`] where you always know each character's position.
    ///
    /// A [*counter string*][0] is a graduated string of arbitrary `length`,
    /// with a `separator` positioned after the immediately preceding number.
    ///
    /// # Examples
    /// ```
    /// use devela::{ascii::AsciiChar, string::StringExt};
    ///
    /// assert_eq!("2*4*6*8*11*14*", String::new_counter(14, AsciiChar::Asterisk));
    /// assert_eq!("_3_5_7_9_12_15_", String::new_counter(15, AsciiChar::LowLine));
    /// ```
    /// [0]: https://www.satisfice.com/blog/archives/22
    #[must_use]
    #[cfg(feature = "ascii")]
    #[cfg_attr(feature = "nightly", doc(cfg(feature = "ascii")))]
    fn new_counter(length: usize, separator: AsciiChar) -> String;
}

#[cfg(feature = "alloc")]
impl StringExt for String {
    #[cfg(feature = "ascii")]
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
