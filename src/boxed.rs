// devela::boxed
//
//! The `Box<T>` type, extends [`alloc::boxed`].
//

/// Brief [`Box`][alloc::boxed::Box] constructor.
///
/// # Examples
/// ```
/// use devela::boxed::bx;
///
/// assert_eq![bx(45), Box::new(45)];
/// ```
#[inline(always)]
#[cfg(feature = "alloc")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "alloc")))]
pub fn bx<T>(v: T) -> alloc::boxed::Box<T> {
    alloc::boxed::Box::new(v)
}
