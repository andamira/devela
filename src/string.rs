// devela::string
//
//! Strings, extends [`alloc::string`].
//

/// Brief [`String`] constructor.
///
/// # Examples
/// ```
/// use devela::all::{iif, S};
///
/// // This
/// let s = iif![2 > 1; S!("string"); S!()];
///
/// // Would be equivalent to
/// let s = if 2 > 1 {
///     String::from("string")
/// } else {
///     "".into()
/// };
///
/// ```
///
#[macro_export]
#[cfg(feature = "alloc")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "alloc")))]
macro_rules! S {
    // new String
    () => {
        String::new()
    };

    // new String from
    ($from:expr) => {
        String::from($from)
    };
}
#[cfg(feature = "alloc")]
pub use S;
