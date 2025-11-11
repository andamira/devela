// devela_base_core::code::default
//
//! Defines the [`ConstDefaultCore`] trait and implements it for many types.
//
// TOC
// - trait ConstDefault

/// The minimal, `no_std` variant of [`ConstDefault`].
///
/// Defines a compile-time default value for fundamental types
/// without depending on `alloc` or `std`.
#[doc = crate::doclink!(custom devela "[`ConstDefault`]" "code/trait.ConstDefault.html")]
pub trait ConstDefaultCore {
    /// Returns the compile-time “default value” for a type.
    const DEFAULT: Self;
}

impl ConstDefaultCore for bool {
    const DEFAULT: Self = false;
}
