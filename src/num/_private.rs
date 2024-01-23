// devela::num::_private
//
//! private numeric helpers
//
// TOC
// - upcasted_op
// - impl_niche

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
                return Err(E::Overflow(None));
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
                return Err(E::Overflow(None));
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
                return Err(E::Overflow(None));
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
                return Err(E::Overflow(None));
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

/// helper macro to implement methods from a wrapper for niche types
//
// $n: the niche name prefix (e.g. NonRange)
// $t: the integer primitive and niche name suffix (e.g. u8)
// $dt: the doclink suffix for the associated method name implemented for the inner primitive
// $($g)*: an optional list of const generics (e.g. RMIN, RMAX)
// $method: the name of the method being implemented
// $self: the self value we're operating on (i.e. self)
// $($arg)*: an optional list of arguments (e.g. min, max, a, b)
macro_rules! impl_niche {
    /* leveraged primitive implementation returns the type without a Result */
    ( Int
      $n:ident : $t:ident : $dt:literal <$($g:ident),*>,
     $(+$const:tt)? $fn:ident, $self:ident $(,)? $($arg:ident : $atype:ty),*
    ) => { paste! {
        #[doc = "*See the [`" $fn "`](#fn." $fn $dt ") implementation for `" $t "`*."]
        /// # Errors
        /// It will return [`Invalid`][crate::num::NumErrors::Invalid]
        /// if the result is invalid for this type.
        #[inline]
        pub $($const)? fn $fn($self, $($arg:$atype),*)
            -> $crate::num::NumResult<Int<[<$n$t:camel>]<$($g,)*>>> {
            let val = Int($self.0.get()).$fn($($arg),*);
            if let Some(res) = [<$n$t:camel>]::<$($g),*>::new(val.0) {
                Ok(Int(res))
            } else {
                Err($crate::num::NumErrors::Invalid)
            }
        }
    }};

    /* leveraged primitive implementation returns the type inside a Result */
    ( Int => res
      $n:ident : $t:ident : $dt:literal <$($g:ident),*>,
      $(+$const:tt)? $fn:ident, $self:ident $(,)? $($arg:ident : $atype:ty),*
    ) => { paste! {
        #[doc = "*See the [`" $fn "`](#fn." $fn $dt ") implementation for `" $t "`*."]
        /// # Errors
        /// It will return [`Invalid`][crate::num::NumErrors::Invalid]
        /// if the result is invalid for this type.
        #[inline]
        pub $($const)? fn $fn($self, $($arg:$atype),*)
            -> $crate::num::NumResult<Int<[<$n$t:camel>]<$($g,)*>>> {
            let fn_res = Int($self.0.get()).$fn($($arg),*);
            match fn_res {
                Err(e) => Err(e),
                Ok(val) => {
                    if let Some(res) = [<$n$t:camel>]::<$($g),*>::new(val.0) {
                        Ok(Int(res))
                    } else {
                        Err($crate::num::NumErrors::Invalid)
                    }
                },
            }
        }
    }};

    // leveraged primitive implementation returns an array of 2 elements
    //
    // used in: div_rem
    ( Int => array2
      $n:ident : $t:ident : $dt:literal <$($g:ident),*>,
      $(+$const:tt)? $fn:ident, $self:ident $(,)? $($arg:ident : $atype:ty),*
    ) => { paste! {
        #[doc = "*See the [`" $fn "`](#fn." $fn $dt ") implementation for `" $t "`*."]
        #[inline]
        pub $($const)? fn $fn($self, $($arg:$atype),*)
            -> $crate::num::NumResult<[Int<[<$n$t:camel>]<$($g,)*>>; 2]> {
            let array = Int($self.0.get()).$fn($($arg),*);
            let a = if let Some(res) = [<$n$t:camel>]::<$($g),*>::new(array[0].0) {
                Int(res) } else { return Err($crate::num::NumErrors::Invalid); };
            let b = if let Some(res) = [<$n$t:camel>]::<$($g),*>::new(array[1].0) {
                Int(res) } else { return Err($crate::num::NumErrors::Invalid); };
            Ok([a, b])
        }
    }};

    // leveraged primitive implementation returns a custom return type directly
    //
    // used in: is_even, is_odd
    ( Int => $ret:ty:
      $n:ident : $t:ident : $dt:literal <$($g:ident),*>,
      $(+$const:tt)? $fn:ident, $self:ident $(,)? $($arg:ident : $atype:ty),*
    ) => { paste! {
        #[doc = "*See the [`" $fn "`](#fn." $fn $dt ") implementation for `" $t "`*."]
        #[inline]
        pub $($const)? fn $fn($self, $($arg:$atype),*) -> $ret {
            Int($self.0.get()).$fn($($arg),*)
        }
    }};
}
pub(crate) use impl_niche;
