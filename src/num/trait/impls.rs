// devela::num::trait::impls
//
//!
//

#[cfg(all(_some_float, not(feature = "std")))]
use crate::code::iif;
use crate::code::paste;
#[cfg(_some_nums)]
use crate::num::{Num, NumResult as Result};
#[cfg(_some_float)]
use core::ops::{Add, Div, Mul, Neg, Rem, Sub};
#[cfg(_some_int)]
use {
    crate::num::{niche::*, NumError},
    NumError::{Invalid, Unspecified},
};

// $p:   the primitive type
// $cap:  the capability feature that enables the given implementation. E.g "_int_i8".
macro_rules! impl_num {
    [] => {
        impl_num![i i8:"_int_i8", i16:"_int_i16", i32:"_int_i32",
            i64:"_int_i64", i128:"_int_i128", isize:"_int_isize"];
        impl_num![u u8:"_int_u8", u16:"_int_u16", u32:"_int_u32",
            u64:"_int_u64", u128:"_int_u128", usize:"_int_usize"];
        impl_num![f f32:"_float_f32", f64:"_float_f64"];
    };

    // Implements `Num` for signed integer types
    // --------------------------------------------------------------------------------------------
    (i $($p:ident : $cap:literal),+) => { $( impl_num![@i $p : $cap]; )+ };
    (@i $p:ident : $cap:literal) => { paste! {
        // i*
        #[cfg(feature = $cap )]
        #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = $cap)))]
        impl Num for $p {
            type Inner = $p;
            type Out = $p;
            type Rhs = $p;

            // base
            #[inline]
            fn num_into(self) -> Self::Inner { self }
            #[inline]
            fn num_from(from: Self::Inner) -> Result<Self> { Ok(from) }
            #[inline]
            fn num_from_ref(from: &Self::Inner) -> Result<Self> { Ok(*from) }
            #[inline]
            fn num_set(&mut self, value: Self::Inner) -> Result<()> { *self = value; Ok(()) }
            #[inline]
            fn num_set_ref(&mut self, value: &Self::Inner) -> Result<()> {
                *self = *value; Ok(())
            }

            // ident
            #[inline]
            fn num_is_zero(&self) -> Result<bool> { Ok(*self == 0) }
            #[inline]
            fn num_is_one(&self) -> Result<bool> { Ok(*self == 1) }
            #[inline]
            fn num_get_zero() -> Result<Self> { Self::num_from(0) }
            #[inline]
            fn num_get_one() -> Result<Self> { Self::num_from(1) }
            #[inline]
            fn num_set_zero(&mut self) -> Result<()> { *self = 0; Ok(()) }
            #[inline]
            fn num_set_one(&mut self) -> Result<()> { *self = 1; Ok(()) }

            // ops
            impl_num![op2_checked Self => add, mul, sub, div, rem];
            impl_num![op1_checked Self => neg, abs];
        }

        // NonZeroI*
        #[cfg(feature = $cap )]
        #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = $cap)))]
        impl Num for [<NonZero $p:camel>] {
            type Inner = $p;
            type Out = [<NonZero $p:camel>];
            type Rhs = [<NonZero $p:camel>];

            // base
            #[inline]
            fn num_into(self) -> Self::Inner { [< NonZero $p:camel >]::get(self) }
            #[inline]
            fn num_from(from: Self::Inner) -> Result<Self> { Self::new(from).ok_or(Invalid) }
            #[inline]
            fn num_from_ref(from: &Self::Inner) -> Result<Self> { Self::new(*from).ok_or(Invalid) }
            #[inline]
            fn num_set(&mut self, value: Self::Inner) -> Result<()> {
                *self = Self::new(value).ok_or(Invalid)?; Ok(())
            }
            #[inline]
            fn num_set_ref(&mut self, value: &Self::Inner) -> Result<()> {
                *self = Self::new(*value).ok_or(Invalid)?; Ok(())
            }

            // ident
            #[inline]
            fn num_is_zero(&self) -> Result<bool> { Ok(false) }
            #[inline]
            fn num_is_one(&self) -> Result<bool> { self.get().num_is_one() }
            #[inline]
            fn num_get_zero() -> Result<Self> { NumError::ni() }
            #[inline]
            fn num_get_one() -> Result<Self> { Ok(Self::new(1).unwrap()) }
            #[inline]
            fn num_set_zero(&mut self) -> Result<()> { NumError::ni() }

            /// # Features
            /// Makes use of the `unsafe_niche` feature if enabled.
            #[inline]
            fn num_set_one(&mut self) -> Result<()> {
                #[cfg(any(feature = "safe_num", not(feature = "unsafe_niche")))]
                { *self = Self::new(1).unwrap(); Ok(()) }

                #[cfg(all(not(feature = "safe_num"), feature = "unsafe_niche"))]
                // SAFETY: we are using a constant
                { *self = unsafe { Self::new_unchecked(1) }; Ok(()) }
            }

            // ops
            impl_num![op2_checked Self => mul];
            impl_num![op1_checked Self => neg, abs];
            impl_num![op2_get_checked Self => add, sub, div, rem];
        }

        // // NonValueI*
        // #[cfg(all(feature = "num_niche_range", feature = "num_niche_impls", feature = $cap))]
        // #[cfg_attr(feature = "nightly_doc",
        //     doc(cfg(all(feature = "num_niche_range", feature = "num_niche_impls", feature = $cap))))]
        // impl<const V: $p> Num for [<NonValue $p:camel>]<V> {
        //     type Inner = $p;
        //     type Out =  [<NonValue $p:camel>]<V>;
        //     type Rhs =  [<NonValue $p:camel>]<V>;
        //     impl_num![custom_i_body];
        // }
        //
        // // NonRangeI*
        // #[cfg(all(feature = "num_niche_range", feature = "num_niche_impls", feature = $cap))]
        // #[cfg_attr(feature = "nightly_doc",
        //     doc(cfg(all(feature = "num_niche_range", feature = "num_niche_impls", feature = $cap))))]
        // impl<const RMIN: $p, const RMAX: $p> Num for [<NonRange $p:camel>]<RMIN, RMAX> {
        //     type Inner = $p;
        //     type Out = [<NonRange $p:camel>]<RMIN, RMAX>;
        //     type Rhs = [<NonRange $p:camel>]<RMIN, RMAX>;
        //     impl_num![custom_i_body];
        // }
        //
        // // RangeI*
        // #[cfg(all(feature = "num_niche_range", feature = "num_niche_impls", feature = $cap))]
        // #[cfg_attr(feature = "nightly_doc",
        //     doc(cfg(all(feature = "num_niche_range", feature = "num_niche_impls", feature = $cap))))]
        // impl<const RMIN: $p, const RMAX: $p> Num for [<Range $p:camel>]<RMIN, RMAX> {
        //     type Inner = $p;
        //     type Out = [<Range $p:camel>]<RMIN, RMAX>;
        //     type Rhs = [<Range $p:camel>]<RMIN, RMAX>;
        //     impl_num![custom_i_body];
        // }
    }};

    // Implements `Num` for unsigned integer types
    // --------------------------------------------------------------------------------------------
    (u $($p:ident : $cap:literal),+) => { $( impl_num![@u $p : $cap]; )+ };
    (@u $p:ident : $cap:literal) => { paste! {
        // u*
        #[cfg(feature = $cap )]
        #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = $cap)))]
        impl Num for $p {
            type Inner = $p;
            type Out = $p;
            type Rhs = $p;

            // base
            #[inline]
            fn num_into(self) -> Self::Inner { self }
            #[inline]
            fn num_from(from: Self::Inner) -> Result<Self> { Ok(from) }
            #[inline]
            fn num_from_ref(from: &Self::Inner) -> Result<Self> { Ok(*from) }
            #[inline]
            fn num_set(&mut self, value: Self::Inner) -> Result<()> { *self = value; Ok(()) }
            #[inline]
            fn num_set_ref(&mut self, value: &Self::Inner) -> Result<()> {
                *self = *value; Ok(())
            }

            // ident
            #[inline]
            fn num_is_zero(&self) -> Result<bool> { Ok(*self == 0) }
            #[inline]
            fn num_is_one(&self) -> Result<bool> { Ok(*self == 1) }
            #[inline]
            fn num_get_zero() -> Result<Self> { Self::num_from(0) }
            #[inline]
            fn num_get_one() -> Result<Self> { Self::num_from(1) }
            #[inline]
            fn num_set_zero(&mut self) -> Result<()> { *self = 0; Ok(()) }
            #[inline]
            fn num_set_one(&mut self) -> Result<()> { *self = 1; Ok(()) }

            // ops
            impl_num![op2_checked Self => add, mul, sub, div, rem];
            impl_num![op1_checked Self => neg];
            #[inline]
            fn num_abs(self) -> Result<Self> { Ok(self) }
            #[inline]
            fn num_ref_abs(&self) -> Result<Self> { Ok(*self) }
        }

        // NonZeroU*
        #[cfg(feature = $cap )]
        #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = $cap)))]
        impl Num for [<NonZero $p:camel>] {
            type Inner = $p;
            type Out = [<NonZero $p:camel>];
            type Rhs = [<NonZero $p:camel>];

            // base
            #[inline]
            fn num_into(self) -> Self::Inner { [< NonZero $p:camel >]::get(self) }
            #[inline]
            fn num_from(from: Self::Inner) -> Result<Self> { Self::new(from).ok_or(Invalid) }
            #[inline]
            fn num_from_ref(from: &Self::Inner) -> Result<Self> { Self::new(*from).ok_or(Invalid) }
            #[inline]
            fn num_set(&mut self, value: Self::Inner) -> Result<()> {
                *self = Self::new(value).ok_or(Invalid)?; Ok(())
            }
            #[inline]
            fn num_set_ref(&mut self, value: &Self::Inner) -> Result<()> {
                *self = Self::new(*value).ok_or(Invalid)?; Ok(())
            }

            // ident
            #[inline]
            fn num_is_zero(&self) -> Result<bool> { Ok(false) }
            #[inline]
            fn num_is_one(&self) -> Result<bool> { Ok(self.get() == 1) }
            #[inline]
            fn num_get_zero() -> Result<Self> { NumError::ni() }
            #[inline]
            fn num_get_one() -> Result<Self> { Ok(Self::new(1).unwrap()) }
            #[inline]
            fn num_set_zero(&mut self) -> Result<()> { NumError::ni() }
            /// # Features
            /// Makes use of the `unsafe_niche` feature if enabled.
            #[inline]
            fn num_set_one(&mut self) -> Result<()> {
                #[cfg(any(feature = "safe_num", not(feature = "unsafe_niche")))]
                { *self = Self::new(1).unwrap(); Ok(()) }

                #[cfg(all(not(feature = "safe_num"), feature = "unsafe_niche"))]
                // SAFETY: we are using a constant
                { *self = unsafe { Self::new_unchecked(1) }; Ok(()) }
            }

            // ops
            impl_num![op2_checked Self => mul]; // add takes an u8 so goes below
            impl_num![op2_get_checked Self => add, sub, div, rem];
            impl_num![op1_none Self => neg]; // no neg for NonZeroU*
            #[inline]
            fn num_abs(self) -> Result<Self> { Ok(self) }
            #[inline]
            fn num_ref_abs(&self) -> Result<Self> { Ok(*self) }
        }

        // // NonValueU*
        // #[cfg(feature = $cap )]
        // #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = $cap)))]
        // #[cfg(feature = "num_niche_impls")]
        // #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "num_niche_impls")))]
        // impl<const V: $p> Num for [<NonValue $p:camel>]<V> {
        //     type Inner = $p;
        //     type Out = [<NonValue $p:camel>]<V>;
        //     type Rhs = [<NonValue $p:camel>]<V>;
        //     impl_num![custom_u_body]; }
        //
        // // NonRangeU*
        // #[cfg(feature = $cap )]
        // #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = $cap)))]
        // #[cfg(all(feature = "num_niche_range", feature = "num_niche_impls"))]
        // #[cfg_attr( feature = "nightly_doc",
        //     doc(cfg(all(feature = "num_niche_range", feature = "num_niche_impls"))))]
        // impl<const RMIN: $p, const RMAX: $p> Num for [<NonRange $p:camel>]<RMIN, RMAX> {
        //     type Inner = $p;
        //     type Out = [<NonRange $p:camel>]<RMIN, RMAX>;
        //     type Rhs = [<NonRange $p:camel>]<RMIN, RMAX>;
        //     impl_num![custom_u_body];
        // }
        //
        // // RangeU*
        // #[cfg(feature = $cap )]
        // #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = $cap)))]
        // #[cfg(all(feature = "num_niche_range", feature = "num_niche_impls"))]
        // #[cfg_attr( feature = "nightly_doc",
        //     doc(cfg(all(feature = "num_niche_range", feature = "num_niche_impls"))))]
        // impl<const RMIN: $p, const RMAX: $p> Num for [<Range $p:camel>]<RMIN, RMAX> {
        //     type Inner = $p;
        //     type Out = [<Range $p:camel>]<RMIN, RMAX>;
        //     type Rhs = [<Range $p:camel>]<RMIN, RMAX>;
        //     impl_num![custom_u_body]; }
    }};

    // Implements `Num` for the floating-point types
    // --------------------------------------------------------------------------------------------
    (f $($p:ident : $cap:literal),+) => { $( impl_num![@f $p : $cap]; )+ };
    (@f $p:ident : $cap:literal) => {
        // f*
        #[cfg(feature = $cap )]
        #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = $cap)))]
        impl Num for $p { paste! {
            type Inner = $p;
            type Out = $p;
            type Rhs = $p;

            // base
            #[inline]
            fn num_into(self) -> Self::Inner { self }
            #[inline]
            fn num_from(from: Self::Inner) -> Result<Self> { Ok(from) }
            #[inline]
            fn num_from_ref(from: &Self::Inner) -> Result<Self> { Ok(*from) }
            #[inline]
            fn num_set(&mut self, value: Self::Inner) -> Result<()> { *self = value; Ok(()) }
            #[inline]
            fn num_set_ref(&mut self, value: &Self::Inner) -> Result<()> {
                *self = *value; Ok(())
            }

            // ident
            #[doc = "This implementation has a tolerance of 5 × [`EPSILON`][" $p "::EPSILON]"]
            #[inline]
            fn num_is_zero(&self) -> Result<bool> {
                Ok(self.num_ref_abs()? < 5.0 * <$p>::EPSILON)
            }
            #[doc = "This implementation has a tolerance of 5 × [`EPSILON`][" $p "::EPSILON]"]
            #[inline]
            fn num_is_one(&self) -> Result<bool> {
                Ok(self.num_sub(1.0)?.num_ref_abs()? < 5.0 * <$p>::EPSILON)
            }
            #[inline]
            fn num_get_zero() -> Result<Self> { Self::num_from(0.0) }
            #[inline]
            fn num_get_one() -> Result<Self> { Self::num_from(1.0) }
            #[inline]
            fn num_set_zero(&mut self) -> Result<()> { *self = Self::num_from(0.0)?; Ok(()) }
            #[inline]
            fn num_set_one(&mut self) -> Result<()> { *self = Self::num_from(1.0)?; Ok(()) }

            // ops
            impl_num![op2_float Self => add, mul, sub, div, rem];
            impl_num![op1_float Self => neg];

            #[inline]
            fn num_abs(self) -> Result<Self> {
                #[cfg(feature = "std")]
                return Ok($p::abs(self));
                #[cfg(not(feature = "std"))]
                Ok(iif![self >= 0.0; self; -self])
            }
            #[inline]
            fn num_ref_abs(&self) -> Result<Self> {
                #[cfg(feature = "std")]
                return Ok($p::abs(*self));
                #[cfg(not(feature = "std"))]
                Ok(iif![*self >= 0.0; *self; -*self])
            }
        }}
    };

    // Inner helpers for the identical body of NonValue, NonRange, InRange
    // with a common body and different ops for signed and unsigned
    // ============================================================================================
    (custom_i_body) => {
        impl_num![custom_body];
        // ops
        impl_num![op2_get_checked Self => add, mul, sub, div, rem];
        impl_num![op1_get_checked Self => neg, abs];
    };
    (custom_u_body) => {
        impl_num![custom_body];
        // ops
        impl_num![op2_get_checked Self => add, mul, sub, div, rem];
        impl_num![op1_get_checked Self => neg];
        #[inline]
        fn num_abs(self) -> Result<Self> { Ok(self) }
        #[inline]
        fn num_ref_abs(&self) -> Result<Self> { Ok(*self) }
    };
    (custom_body) => {
        // base
        #[inline]
        fn num_into(self) -> Self::Inner { self.get() }
        #[inline]
        fn num_from(from: Self::Inner) -> Result<Self> { Self::new(from).ok_or(Invalid) }
        #[inline]
        fn num_from_ref(from: &Self::Inner) -> Result<Self> { Self::new(*from).ok_or(Invalid) }
        #[inline]
        fn num_set(&mut self, value: Self::Inner) -> Result<()> {
            *self = Self::num_from(value)?; Ok(())
        }
        #[inline]
        fn num_set_ref(&mut self, value: &Self::Inner) -> Result<()> {
            *self = Self::num_from(*value)?; Ok(())
        }

        // ident
        #[inline]
        fn num_is_zero(&self) -> Result<bool> { Ok(self.get() == 0) }
        #[inline]
        fn num_is_one(&self) -> Result<bool> { Ok(self.get() == 1) }
        #[inline]
        fn num_get_zero() -> Result<Self> { Self::num_from(0) }
        #[inline]
        fn num_get_one() -> Result<Self> { Self::num_from(1) }
        #[inline]
        fn num_set_zero(&mut self) -> Result<()> { *self = Self::num_from(0)?; Ok(()) }
        #[inline]
        fn num_set_one(&mut self) -> Result<()> { *self = Self::num_from(1)?; Ok(()) }
    };

    // Inner helpers for unary and binary ops
    // ============================================================================================

    /* ops that returns `NotImplemented` */

    // (this could be regarded as unnecessary since it's the same as the default implementantion,
    // but it allows us to debug missing implementations while swithing the commented out blocks
    // in the num module that provides non-automatic implementations for the trait methods)

    (op1_none $Self:ty => $($op:ident),+) => {
        // $( impl_num![@op1_none $Self => $op]; )+ // uncomment to DEBUG
    };
    (@op1_none $Self:ty => $op:ident) => { paste! {
        #[inline]
        fn [<num_ $op>](self) -> Result<$Self::Out> { NumError::ni() }
        #[inline]
        fn [<num_ref_ $op>](&self) -> Result<$Self::Out> { NumError::ni() }
    }};
    (op2_none $Self:ty => $($op:ident),+) => {
        $( impl_num![@op2_none $Self => $op]; )+ };
    (@op2_none $Self:ty => $op:ident) => {
        #[inline]
        fn [<num_ $op>](self, other: $Self) -> Result<$Self::Out> { NumError::ni() }
        #[inline]
        fn [<num_ref_ $op>](&self, other: &$Self) -> Result<$Self::Out> { NumError::ni() }
        #[inline]
        fn [<num_ref_ $op _assign>](&mut self, other: &$Self) -> Result<()> { NumError::ni() }
    };

    /* ops that call .checked() for i*, u*, and few for NonZero* */

    (op1_checked $Self:ty => $($op:ident),+) => {
        $( impl_num![@op1_checked $Self => $op]; )+ };
    (@op1_checked $Self:ty => $op:ident) => { paste! {
        #[inline]
        fn [<num_ $op>](self) -> Result<$Self::Out> {
            self.[<checked_$op>]().ok_or(Unspecified)
        }
        #[inline]
        fn [<num_ref_ $op>](&self) -> Result<$Self::Out> {
            self.[<checked_$op>]().ok_or(Unspecified)
        }
    }};
    (op2_checked $Self:ty => $($op:ident),+) => {
        $( impl_num![@op2_checked $Self => $op]; )+ };
    (@op2_checked $Self:ty => $op:ident) => { paste! {
        #[inline]
        fn [<num_ $op>](self, other: $Self) -> Result<$Self::Out> {
            self.[<checked_ $op>](other).ok_or(Unspecified)
        }
        #[inline]
        fn [<num_ref_ $op>](&self, other: &$Self) -> Result<$Self::Out> {
            self.[<checked_ $op>](*other).ok_or(Unspecified)
        }
        #[inline]
        fn [<num_ref_ $op _assign>](&mut self, other: &$Self) -> Result<()> {
            *self = self.[<checked_ $op>](*other).ok_or(Unspecified)?;
            Ok(())
        }
    }};

    /* ops that call .get().checked() for: NonZero*, NonValue*, [Non|In]Range* */

    (op1_get_checked $Self:ty => $($op:ident),+) => {
        $( impl_num![@op1_get_checked $Self => $op]; )+ };
    (@op1_get_checked $Self:ty => $op:ident) => { paste! {
        #[inline]
        fn [<num_ $op>](self) -> Result<$Self::Out> {
            $Self::new(self.get().[<checked_ $op>]().ok_or(Unspecified)?).ok_or(Unspecified)
        }
        #[inline]
        fn [<num_ref_ $op>](&self) -> Result<$Self::Out> {
            $Self::new(self.get().[<checked_ $op>]().ok_or(Unspecified)?).ok_or(Unspecified)
        }
    }};
    (op2_get_checked $Self:ty => $($op:ident),+) => {
        $( impl_num![@op2_get_checked $Self => $op]; )+ };
    (@op2_get_checked $Self:ty => $op:ident) => { paste! {
        #[inline]
        fn [<num_ $op>](self, other: $Self) -> Result<$Self::Out> {
            $Self::new(self.get().[<checked_ $op>](other.get()).ok_or(Unspecified)?)
                .ok_or(Unspecified)
        }
        #[inline]
        fn [<num_ref_ $op>](&self, other: &$Self) -> Result<$Self::Out> {
            $Self::new(self.get().[<checked_ $op>](other.get()).ok_or(Unspecified)?)
                .ok_or(Unspecified)
        }
        #[inline]
        fn [<num_ref_ $op _assign>](&mut self, other: &$Self) -> Result<()> {
            *self = $Self::new(self.get().[<checked_ $op>](other.get()).ok_or(Unspecified)?)
                .ok_or(Unspecified)?;
            Ok(())
        }
    }};

    /* ops for floating-point f* types */

    (op1_float $Self:ty => $($op:ident),+) => { $( impl_num![@op1_float $Self => $op]; )+ };
    (@op1_float $Self:ty => $op:ident) => { paste! {
        #[inline]
        fn [<num_ $op>](self) -> Result<$Self::Out> { Ok([<$op:camel>]::[<$op>](self)) }
        #[inline]
        fn [<num_ref_ $op>](&self) -> Result<$Self::Out> { Ok([<$op:camel>]::[<$op>](self)) }
    }};
    (op2_float $Self:ty => $($op:ident),+) => { $( impl_num![@op2_float $Self => $op]; )+ };
    (@op2_float $Self:ty => $op:ident) => { paste! {
        #[inline]
        fn [<num_ $op>](self, other: $Self) -> Result<$Self::Out> {
            Ok([<$op:camel>]::[<$op>](self, other))
        }
        #[inline]
        fn [<num_ref_ $op>](&self, other: &$Self) -> Result<$Self::Out> {
            Ok([<$op:camel>]::[<$op>](self, *other))
        }
    }};

    // Inner helpers for identities
    // ============================================================================================
    // ...
}
impl_num!();
