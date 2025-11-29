// devela_base_num::num::_internals
//
//! Workspace-internal numeric helpers: `upcasted_op`.
//

#![allow(unused, non_camel_case_types)]

/// helper macro to only do checked operations when we can't upcast (i.e. for 128-bits).
///
/// Performs checked operations only if the upcasted type is the same
/// as the non-upcasted one. It is compatible with const functions.
///
/// # Arguments
/// `$lhs`:      the left hand side operator
/// `$rhs`:      the right hand side operator
/// `$modulus`:  the modulus (for reduced_* ops)
/// `$ba`:       the base type
/// `$up`:       the upcasted type
///
/// # Invoked from:
/// - num/int/wrapper/impl_root.rs
///
/// # Examples
/// ```ignore
/// let sum = upcasted_op![add_err(v, m) i32 => i64];
/// let sum = upcasted_op![reduced_add_err(v, m) % 40; i32 => i64];
/// ```
/// # Features
/// It makes use of `unsafe_hint` to optimize arithmetic ops when able to upcast.
//
// TODO IMPROVE: try to unify with impl_modulo::upcastop
#[doc(hidden)]
#[macro_export]
macro_rules! upcasted_op {
    (
    /* basic arithmetic ops */
    // if we've not upcasted, do checked operation and return err on overflow
    add_err($lhs:expr, $rhs:expr) $ba:ty => $up:ty) => {
        if $crate::cif!(diff($ba, $up)) {
            #[cfg(any(base_safe, not(feature = "unsafe_hint")))]
            {
                $lhs + $rhs
            }
            #[cfg(all(not(base_safe), feature = "unsafe_hint"))]
            // SAFETY: can't overflow if upcasted
            unsafe {
                $lhs.unchecked_add($rhs)
            }
        } else {
            if let Some(sum) = $lhs.checked_add($rhs) {
                sum
            } else {
                return Err($crate::IntError::Overflow(None));
            }
        }
    };
    (mul_err($lhs:expr, $rhs:expr) $ba:ty => $up:ty) => {
        if $crate::cif!(diff($ba, $up)) {
            #[cfg(any(base_safe, not(feature = "unsafe_hint")))]
            {
                $lhs * $rhs
            }
            #[cfg(all(not(base_safe), feature = "unsafe_hint"))]
            // SAFETY: can't overflow if upcasted
            unsafe {
                $lhs.unchecked_mul($rhs)
            }
        } else {
            if let Some(product) = $lhs.checked_mul($rhs) {
                product
            } else {
                return Err($crate::IntError::Overflow(None));
            }
        }
    };
    (
    /* reduced (modulo) ops */

    // if we've not upcasted, first reduce the sumands with the given modulus,
    // then do checked operation and return err on overflow
    reduced_add_err($lhs:expr, $rhs:expr) % $modulus:expr; $ba:ty => $up:ty) => {
        if $crate::cif!(diff($ba, $up)) {
            #[cfg(any(base_safe, not(feature = "unsafe_hint")))]
            {
                $lhs + $rhs
            }
            #[cfg(all(not(base_safe), feature = "unsafe_hint"))]
            // SAFETY: can't overflow if upcasted
            unsafe {
                $lhs.unchecked_add($rhs)
            }
        } else {
            // reduce each sumand before checked operation
            if let Some(sum) = ($lhs % $modulus).checked_add($rhs % $modulus) {
                sum
            } else {
                return Err($crate::IntError::Overflow(None));
            }
        }
    };
    (
    // if we've not upcasted, just reduce the sumands with the given $modulus
    reduced_add($lhs:expr, $rhs:expr) % $modulus:expr; $ba:ty => $up:ty) => {
        if $crate::cif!(diff($ba, $up)) {
            #[cfg(any(base_safe, not(feature = "unsafe_hint")))]
            {
                $lhs + $rhs
            }
            #[cfg(all(not(base_safe), feature = "unsafe_hint"))]
            // SAFETY: can't overflow if upcasted
            unsafe {
                $lhs.unchecked_add($rhs)
            }
        } else {
            // reduce each operand before the operation that could panic
            ($lhs % $modulus) + ($rhs % $modulus)
        }
    };
    (reduced_mul_err($lhs:expr, $rhs:expr) % $modulus:expr; $ba:ty => $up:ty) => {
        if $crate::cif!(diff($ba, $up)) {
            #[cfg(any(base_safe, not(feature = "unsafe_hint")))]
            {
                $lhs * $rhs
            }
            #[cfg(all(not(base_safe), feature = "unsafe_hint"))]
            // SAFETY: can't overflow if upcasted
            unsafe {
                $lhs.unchecked_mul($rhs)
            }
        } else {
            // reduce each factor before checked operation
            if let Some(product) = ($lhs % $modulus).checked_mul($rhs % $modulus) {
                product
            } else {
                return Err($crate::IntError::Overflow(None));
            }
        }
    };
    (reduced_mul($lhs:expr, $rhs:expr) % $modulus:expr; $ba:ty => $up:ty) => {
        if $crate::cif!(diff($ba, $up)) {
            #[cfg(any(base_safe, not(feature = "unsafe_hint")))]
            {
                $lhs * $rhs
            }
            #[cfg(all(not(base_safe), feature = "unsafe_hint"))]
            // SAFETY: can't overflow if upcasted
            unsafe {
                $lhs.unchecked_mul($rhs)
            }
        } else {
            // reduce each operand before the operation that could panic
            ($lhs % $modulus) + ($rhs % $modulus)
        }
    };
}
#[doc(hidden)]
pub use upcasted_op;
