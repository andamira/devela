// devela_base_core::num::_internals
//
//! Workspace-internal numeric helpers: `impl_ops`.
//

#![allow(unused, non_camel_case_types)]

/// implement the arithmetic operators for a unit struct wrapper, based on the inner type
///
/// # Arguments:
/// $W:   the outer wrapper
/// $T:   the inner type
/// $cap: the (optional) capability feature that enables the given implementation. E.g "_int_i8".
///
/// # Invoked from:
/// - int/wrapper/mod.rs
/// - float/wrapper/mod.rs
#[doc(hidden)]
#[macro_export]
#[allow(clippy::crate_in_macro_def, reason = "paste! must be in the root of invoking crate")]
macro_rules! impl_ops {
    ($W:ident: $($T:ty $( : $cap:literal)? ),+) => { $(
        $crate::impl_ops![@common $W($T $(:$cap)? )];
        $crate::impl_ops![@neg $W($T $(:$cap)? )];
    )+ };
    ($W:ident: (no_neg) $($T:ty $(: $cap:literal)? ),+) => { $(
        $crate::impl_ops![@common $W($T $(:$cap)? )];
    )+ };

    (@common $W:ident($T:ty $(: $cap:literal)? )) => {
        $crate::impl_ops![@op $W($T $(:$cap)? ), Add, add];
        $crate::impl_ops![@op $W($T $(:$cap)? ), Sub, sub];
        $crate::impl_ops![@op $W($T $(:$cap)? ), Mul, mul];
        $crate::impl_ops![@op $W($T $(:$cap)? ), Div, div];
        $crate::impl_ops![@op $W($T $(:$cap)? ), Rem, rem];
        $crate::impl_ops![@op_assign $W($T $(:$cap)? ), AddAssign, add_assign];
        $crate::impl_ops![@op_assign $W($T $(:$cap)? ), SubAssign, sub_assign];
        $crate::impl_ops![@op_assign $W($T $(:$cap)? ), MulAssign, mul_assign];
        $crate::impl_ops![@op_assign $W($T $(:$cap)? ), DivAssign, div_assign];
        $crate::impl_ops![@op_assign $W($T $(:$cap)? ), RemAssign, rem_assign];
    };
    (@neg $W:ident($T:ty $(: $cap:literal)? )) => {
        $( #[cfg(feature = $cap )] #[cfg_attr(nightly_doc, doc(cfg(feature = $cap)))] )?
        impl core::ops::Neg for $W<$T> {
            type Output = $W<$T>;
            fn neg(self) -> $W<$T> { $W(self.0.neg()) }
        }
    };

    (
    // $wrap:  the wrapper type
    // $T:     the inner type
    // $trait: the trait to implement
    // $fn:    the name of the method
        @op $W:ident($T:ty $(: $cap:literal)? ), $trait:ident, $fn:ident) => {
        /* $W<$T> op $W<$T> -> $W<$T> */
        $( #[cfg(feature = $cap )] #[cfg_attr(nightly_doc, doc(cfg(feature = $cap)))] )?
        impl core::ops::$trait for $W<$T> {
            $crate::impl_ops![@op_body $W($T), $fn, $W<$T>, 0];
        }
        $( #[cfg(feature = $cap )] #[cfg_attr(nightly_doc, doc(cfg(feature = $cap)))] )?
        impl<'s> core::ops::$trait<$W<$T>> for &'s $W<$T> {
            $crate::impl_ops![@op_body $W($T), $fn, $W<$T>, 0];
        }
        $( #[cfg(feature = $cap )] #[cfg_attr(nightly_doc, doc(cfg(feature = $cap)))] )?
        impl<'o> core::ops::$trait<&'o $W<$T>> for $W<$T> {
            $crate::impl_ops![@op_body $W($T), $fn, &'o $W<$T>, 0];
        }
        $( #[cfg(feature = $cap )] #[cfg_attr(nightly_doc, doc(cfg(feature = $cap)))] )?
        impl<'s, 'o> core::ops::$trait<&'o $W<$T>> for &'s $W<$T> {
            $crate::impl_ops![@op_body $W($T), $fn, &'o $W<$T>, 0];
        }
        /* $W<$T> op $T -> $W<$T> */
        $( #[cfg(feature = $cap )] #[cfg_attr(nightly_doc, doc(cfg(feature = $cap)))] )?
        impl core::ops::$trait<$T> for $W<$T> {
            $crate::impl_ops![@op_body $W($T), $fn, $T];
        }
        $( #[cfg(feature = $cap )] #[cfg_attr(nightly_doc, doc(cfg(feature = $cap)))] )?
        impl<'s> core::ops::$trait<$T> for &'s $W<$T> {
            $crate::impl_ops![@op_body $W($T), $fn, $T];
        }
        $( #[cfg(feature = $cap )] #[cfg_attr(nightly_doc, doc(cfg(feature = $cap)))] )?
        impl<'o> core::ops::$trait<&'o $T> for $W<$T> {
            $crate::impl_ops![@op_body $W($T), $fn, &'o $T];
        }
        $( #[cfg(feature = $cap )] #[cfg_attr(nightly_doc, doc(cfg(feature = $cap)))] )?
        impl<'s, 'o> core::ops::$trait<&'o $T> for &'s $W<$T> {
            $crate::impl_ops![@op_body $W($T), $fn, &'o $T];
        }
    };
    (@op_body $W:ident($T:ty), $fn:ident, $other:ty $(, $other_field:tt)?) => {
        type Output = $W<$T>;
        fn $fn(self, other: $other) -> $W<$T> { $W(self.0.$fn(other$(. $other_field)?)) }
    };

    (@op_assign $W:ident($T:ty $(: $cap:literal)?), $trait:ident, $fn:ident) => { crate::paste! {
        /* $W<$T> op_assign $W<$T> */
        $( #[cfg(feature = $cap )] #[cfg_attr(nightly_doc, doc(cfg(feature = $cap)))] )?
        impl core::ops::$trait for $W<$T> {
            fn $fn(&mut self, other: $W<$T>) { self.0.$fn(other.0); }
        }
        $( #[cfg(feature = $cap )] #[cfg_attr(nightly_doc, doc(cfg(feature = $cap)))] )?
        impl<'o> core::ops::$trait<&'o $W<$T>> for $W<$T> {
            fn $fn(&mut self, other: &'o $W<$T>) { self.0.$fn(other.0); }
        }
        /* $W<$T> op_assign $T -> $W<$T> */
        $( #[cfg(feature = $cap )] #[cfg_attr(nightly_doc, doc(cfg(feature = $cap)))] )?
        impl core::ops::$trait<$T> for $W<$T> {
            fn $fn(&mut self, other: $T) { self.0.$fn(other); }
        }
        $( #[cfg(feature = $cap )] #[cfg_attr(nightly_doc, doc(cfg(feature = $cap)))] )?
        impl<'o> core::ops::$trait<&'o $T> for $W<$T> {
            fn $fn(&mut self, other: &'o $T) { self.0.$fn(other); }
        }
    }};
}
#[doc(hidden)]
pub use impl_ops;
