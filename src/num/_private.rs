// devela::num::_private
//
//! private numeric helpers
//

#![allow(unused)]

/* upcasted isize|usize aliases */

#[cfg(target_pointer_width = "8")]
pub(crate) type UpcastedIsize = i16;
#[cfg(target_pointer_width = "16")]
pub(crate) type UpcastedIsize = i32;
#[cfg(target_pointer_width = "32")]
pub(crate) type UpcastedIsize = i64;
#[cfg(target_pointer_width = "64")]
pub(crate) type UpcastedIsize = i128;
#[cfg(target_pointer_width = "128")]
pub(crate) type UpcastedIsize = isize;
#[cfg(target_pointer_width = "8")]
pub(crate) type UpcastedUsize = u16;
#[cfg(target_pointer_width = "16")]
pub(crate) type UpcastedUsize = u32;
#[cfg(target_pointer_width = "32")]
pub(crate) type UpcastedUsize = u64;
#[cfg(target_pointer_width = "64")]
pub(crate) type UpcastedUsize = u128;
#[cfg(target_pointer_width = "128")]
pub(crate) type UpcastedUsize = usize;

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
/// # Examples
/// ```ignore
/// let sum = upcasted_op![add_err(v, m) i32 => i64];
/// let sum = upcasted_op![reduced_add_err(v, m) % 40; i32 => i64];
/// ```
// WAIT: [unchecked_add|mul](https://github.com/rust-lang/rust/issues/85122)
macro_rules! upcasted_op {
    /* basic arithmetic ops */
    // if we've not upcasted, do checked operation and return err on overflow
    (add_err($lhs:expr, $rhs:expr) $ba:ty => $up:ty) => {
        if $crate::code::cif!(diff($ba, $up)) {
            $lhs + $rhs
        } else {
            if let Some(sum) = $lhs.checked_add($rhs) {
                sum
            } else {
                return Err(E::Overflow);
            }
        }
    };
    (mul_err($lhs:expr, $rhs:expr) $ba:ty => $up:ty) => {
        if $crate::code::cif!(diff($ba, $up)) {
            $lhs * $rhs
        } else {
            if let Some(product) = $lhs.checked_mul($rhs) {
                product
            } else {
                return Err(E::Overflow);
            }
        }
    };

    /* reduced (modulo) ops */
    // if we've not upcasted, first reduce the sumands with the given modulus,
    // then do checked operation and return err on overflow
    (reduced_add_err($lhs:expr, $rhs:expr) % $modulus:expr; $ba:ty => $up:ty) => {
        if $crate::code::cif!(diff($ba, $up)) {
            $lhs + $rhs
        } else {
            // reduce each sumand before checked operation
            if let Some(sum) = ($lhs % $modulus).checked_add($rhs % $modulus) {
                sum
            } else {
                return Err(E::Overflow);
            }
        }
    };
    // if we've not upcasted, just reduce the sumands with the given $modulus
    (reduced_add($lhs:expr, $rhs:expr) % $modulus:expr; $ba:ty => $up:ty) => {
        if $crate::code::cif!(diff($ba, $up)) {
            $lhs + $rhs
        } else {
            // reduce each operand before the operation that could panic
            ($lhs % $modulus) + ($rhs % $modulus)
        }
    };
    (reduced_mul_err($lhs:expr, $rhs:expr) % $modulus:expr; $ba:ty => $up:ty) => {
        if $crate::code::cif!(diff($ba, $up)) {
            $lhs * $rhs
        } else {
            // reduce each factor before checked operation
            if let Some(product) = ($lhs % $modulus).checked_mul($rhs % $modulus) {
                product
            } else {
                return Err(E::Overflow);
            }
        }
    };
    (reduced_mul($lhs:expr, $rhs:expr) % $modulus:expr; $ba:ty => $up:ty) => {
        if $crate::code::cif!(diff($ba, $up)) {
            $lhs * $rhs
        } else {
            // reduce each operand before the operation that could panic
            ($lhs % $modulus) + ($rhs % $modulus)
        }
    };
}
pub(crate) use upcasted_op;
