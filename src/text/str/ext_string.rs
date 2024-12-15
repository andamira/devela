// devela::text::ext::string
//
//!
//

#[cfg(feature = "alloc")]
use crate::text::{String, ToString};

/// Marker trait to prevent downstream implementations of the [`ExtString`] trait.
#[cfg(feature = "alloc")]
trait Sealed {}
#[cfg(feature = "alloc")]
impl Sealed for String {}

/// Extension trait providing additional methods for [`String`].
#[expect(private_bounds, reason = "Sealed")]
#[cfg(feature = "alloc")]
#[cfg_attr(feature = "nightly_doc", doc(notable_trait, cfg(feature = "alloc")))]
pub trait ExtString: Sealed {
    /// Returns a [`String`] where you always know each character's position.
    ///
    /// A [*counter string*][0] is a graduated string of arbitrary `length`,
    /// with a `separator` positioned after the immediately preceding number.
    ///
    /// # Examples
    /// ```
    /// use devela::ExtString;
    ///
    /// assert_eq!("2*4*6*8*11*14*", String::new_counter(14, '*'));
    /// assert_eq!("_3_5_7_9_12_15_", String::new_counter(15, '_'));
    /// ```
    /// [0]: https://www.satisfice.com/blog/archives/22
    #[must_use]
    fn new_counter(length: usize, separator: char) -> String;
}

#[cfg(feature = "alloc")]
impl ExtString for String {
    fn new_counter(mut length: usize, separator: char) -> String {
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
