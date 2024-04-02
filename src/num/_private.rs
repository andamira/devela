// devela::num::_private
//
//! private numeric helpers
//
// TOC
// - upcasted_op
// - impl_ops

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
                return Err($crate::num::NumError::Overflow(None));
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
                return Err($crate::num::NumError::Overflow(None));
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
                return Err($crate::num::NumError::Overflow(None));
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
                return Err($crate::num::NumError::Overflow(None));
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

// implement the arithmetic operators for a unit struct wrapper, based on the inner type
//
// # Arguments:
// $W:   the outer wrapper
// $T:   the inner type
// $cap: the capability feature that enables the given implementation. E.g "_i8".
//
// # Invoked from:
// - num/int/wrapper/mod.rs
// - num/float/wrapper/mod.rs
macro_rules! impl_ops {
    ($W:ident: $($T:ty : $cap:literal ),+) => { $(
        $crate::num::impl_ops![@common $W($T:$cap)];
        $crate::num::impl_ops![@neg $W($T:$cap)];
    )+ };
    ($W:ident: (no_neg) $($T:ty : $cap:literal),+) => { $(
        $crate::num::impl_ops![@common $W($T:$cap)];
    )+ };

    (@common $W:ident($T:ty : $cap:literal)) => {
        $crate::num::impl_ops![@op $W($T:$cap), Add, add];
        $crate::num::impl_ops![@op $W($T:$cap), Sub, sub];
        $crate::num::impl_ops![@op $W($T:$cap), Mul, mul];
        $crate::num::impl_ops![@op $W($T:$cap), Div, div];
        $crate::num::impl_ops![@op $W($T:$cap), Rem, rem];
        $crate::num::impl_ops![@op_assign $W($T:$cap), AddAssign, add_assign];
        $crate::num::impl_ops![@op_assign $W($T:$cap), SubAssign, sub_assign];
        $crate::num::impl_ops![@op_assign $W($T:$cap), MulAssign, mul_assign];
        $crate::num::impl_ops![@op_assign $W($T:$cap), DivAssign, div_assign];
        $crate::num::impl_ops![@op_assign $W($T:$cap), RemAssign, rem_assign];
    };
    (@neg $W:ident($T:ty : $cap:literal)) => {
        #[cfg(feature = $cap )]
        #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = $cap)))]
        impl core::ops::Neg for $W<$T> {
            type Output = $W<$T>;
            #[inline] fn neg(self) -> $W<$T> { $W(self.0.neg()) }
        }
    };

    // $wrap:  the wrapper type
    // $T:     the inner type
    // $trait: the trait to implement
    // $fn:    the name of the method
    (@op $W:ident($T:ty : $cap:literal), $trait:ident, $fn:ident) => {
        /* $W<$T> op $W<$T> -> $W<$T> */
        #[cfg(feature = $cap )]
        #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = $cap)))]
        impl core::ops::$trait for $W<$T> {
            $crate::num::impl_ops![@op_body $W($T), $fn, $W<$T>, 0];
        }
        #[cfg(feature = $cap )]
        #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = $cap)))]
        impl<'s> core::ops::$trait<$W<$T>> for &'s $W<$T> {
            $crate::num::impl_ops![@op_body $W($T), $fn, $W<$T>, 0];
        }
        #[cfg(feature = $cap )]
        #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = $cap)))]
        impl<'o> core::ops::$trait<&'o $W<$T>> for $W<$T> {
            $crate::num::impl_ops![@op_body $W($T), $fn, &'o $W<$T>, 0];
        }
        #[cfg(feature = $cap )]
        #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = $cap)))]
        impl<'s, 'o> core::ops::$trait<&'o $W<$T>> for &'s $W<$T> {
            $crate::num::impl_ops![@op_body $W($T), $fn, &'o $W<$T>, 0];
        }
        /* $W<$T> op $T -> $W<$T> */
        #[cfg(feature = $cap )]
        #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = $cap)))]
        impl core::ops::$trait<$T> for $W<$T> {
            $crate::num::impl_ops![@op_body $W($T), $fn, $T];
        }
        #[cfg(feature = $cap )]
        #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = $cap)))]
        impl<'s> core::ops::$trait<$T> for &'s $W<$T> {
            $crate::num::impl_ops![@op_body $W($T), $fn, $T];
        }
        #[cfg(feature = $cap )]
        #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = $cap)))]
        impl<'o> core::ops::$trait<&'o $T> for $W<$T> {
            $crate::num::impl_ops![@op_body $W($T), $fn, &'o $T];
        }
        #[cfg(feature = $cap )]
        #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = $cap)))]
        impl<'s, 'o> core::ops::$trait<&'o $T> for &'s $W<$T> {
            $crate::num::impl_ops![@op_body $W($T), $fn, &'o $T];
        }
    };
    (@op_body $W:ident($T:ty), $fn:ident, $other:ty $(, $other_field:tt)?) => {
        type Output = $W<$T>;
        #[inline]
        fn $fn(self, other: $other) -> $W<$T> { $W(self.0.$fn(other$(. $other_field)?)) }
    };

    (@op_assign $W:ident($T:ty : $cap:literal), $trait:ident, $fn:ident) => { $crate::paste! {
        /* $W<$T> op_assign $W<$T> */
        #[cfg(feature = $cap )]
        #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = $cap)))]
        impl core::ops::$trait for $W<$T> {
            #[inline] fn $fn(&mut self, other: $W<$T>) { self.0.$fn(other.0); }
        }
        #[cfg(feature = $cap )]
        #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = $cap)))]
        impl<'o> core::ops::$trait<&'o $W<$T>> for $W<$T> {
            #[inline] fn $fn(&mut self, other: &'o $W<$T>) { self.0.$fn(other.0); }
        }
        /* $W<$T> op_assign $T -> $W<$T> */
        #[cfg(feature = $cap )]
        #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = $cap)))]
        impl core::ops::$trait<$T> for $W<$T> {
            #[inline] fn $fn(&mut self, other: $T) { self.0.$fn(other); }
        }
        #[cfg(feature = $cap )]
        #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = $cap)))]
        impl<'o> core::ops::$trait<&'o $T> for $W<$T> {
            #[inline] fn $fn(&mut self, other: &'o $T) { self.0.$fn(other); }
        }
    }};
}
pub(crate) use impl_ops;
