// devela::num::_private
//
//! private numeric helpers
//
// TOC
// - upcasted_op
// - num_niche_impls

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
macro_rules! num_niche_impls {
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

    /* ... */

    // implements locally defined Int methods for all niche types.
    (impl_int $arm:ident) => {
        num_niche_impls![impl_int_signed $arm];
        num_niche_impls![impl_int_unsigned $arm];
    };
    (impl_int_signed $arm:ident) => {
        impl_int![$arm
            NonZero:i8<>:"-18":"", NonZero:i16<>:"-19":"-1",
            NonZero:i32<>:"-20":"-2", NonZero:i64<>:"-21":"-3",
            NonZero:i128<>:"-22":"-4", NonZero:isize<>:"-23":"-5",
            NonSpecific:i8<V>:"-30":"", NonSpecific:i16<V>:"-31":"-1",
            NonSpecific:i32<V>:"-32":"-2", NonSpecific:i64<V>:"-33":"-3",
            NonSpecific:i128<V>:"-34":"-4", NonSpecific:isize<V>:"-35":"-5",
            NonRange:i8<RMIN,RMAX>:"-42":"", NonRange:i16<RMIN,RMAX>:"-43":"-1",
            NonRange:i32<RMIN,RMAX>:"-44":"-2", NonRange:i64<RMIN,RMAX>:"-45":"-3",
            NonRange:i128<RMIN,RMAX>:"-46":"-4", NonRange:isize<RMIN,RMAX>:"-47":"-5",
            Range:i8<RMIN,RMAX>:"-54":"", Range:i16<RMIN,RMAX>:"-55":"-1",
            Range:i32<RMIN,RMAX>:"-56":"-2", Range:i64<RMIN,RMAX>:"-57":"-3",
            Range:i128<RMIN,RMAX>:"-58":"-4", Range:isize<RMIN,RMAX>:"-59":"-5",
        ];

    };
    (impl_int_unsigned $arm:ident) => {
        impl_int![$arm
            NonZero:u8<>:"-12":"-6", NonZero:u16<>:"-13":"-7",
            NonZero:u32<>:"-14":"-8", NonZero:u64<>:"-15":"-9",
            NonZero:u128<>:"-16":"-10", NonZero:usize<>:"-17":"-11",
            NonSpecific:u8<V>:"-24":"-6", NonSpecific:u16<V>:"-25":"-7",
            NonSpecific:u32<V>:"-26":"-8", NonSpecific:u64<V>:"-27":"-9",
            NonSpecific:u128<V>:"-28":"-10", NonSpecific:usize<V>:"-29":"-11",
            NonRange:u8<RMIN,RMAX>:"-36":"-6", NonRange:u16<RMIN,RMAX>:"-37":"-7",
            NonRange:u32<RMIN,RMAX>:"-38":"-8", NonRange:u64<RMIN,RMAX>:"-39":"-9",
            NonRange:u128<RMIN,RMAX>:"-40":"-10", NonRange:usize<RMIN,RMAX>:"-41":"11",
            Range:u8<RMIN,RMAX>:"-48":"-6", Range:u16<RMIN,RMAX>:"-49":"-7",
            Range:u32<RMIN,RMAX>:"-50":"-8", Range:u64<RMIN,RMAX>:"-51":"-9",
            Range:u128<RMIN,RMAX>:"-52":"-10", Range:usize<RMIN,RMAX>:"-53":"-11",
        ];
    };
}
pub(crate) use num_niche_impls;
