// devela::num::traits::num::impls
//
//!
//

#[cfg(not(feature = "std"))]
use crate::codegen::iif;
use crate::codegen::paste;
use crate::num::all::*;
use core::ops::{Add, Div, Mul, Neg, Rem, Sub};

impl_num![];
macro_rules! impl_num {
    [] => {
        impl_num![i i8, i16, i32, i64, i128, isize];
        impl_num![u u8, u16, u32, u64, u128, usize];
        impl_num![f f32, f64];
    };

    // Implements `Num` for signed integer types
    // --------------------------------------------------------------------------------------------
    (i $($p:ident),+) => { $( impl_num![@i $p]; )+ };
    (@i $p:ident) => { paste! {
        // i*
        impl Num for $p {
            type Inner = $p;
            type OpOut = $p;

            // base
            #[inline]
            fn num_into(self) -> Self::Inner { self }
            #[inline]
            fn num_from(from: Self::Inner) -> Option<Self> { Some(from) }
            #[inline]
            fn num_from_ref(from: &Self::Inner) -> Option<Self> { Some(*from) }
            #[inline]
            fn num_set(&mut self, value: Self::Inner) -> Option<()> { *self = value; Some(()) }
            #[inline]
            fn num_set_ref(&mut self, value: &Self::Inner) -> Option<()> {
                *self = *value; Some(())
            }

            // ident
            #[inline]
            fn num_is_zero(&self) -> Option<bool> { Some(*self == 0) }
            #[inline]
            fn num_is_one(&self) -> Option<bool> { Some(*self == 1) }
            #[inline]
            fn num_get_zero() -> Option<Self> { Self::num_from(0) }
            #[inline]
            fn num_get_one() -> Option<Self> { Self::num_from(1) }
            #[inline]
            fn num_set_zero(&mut self) -> Option<()> { *self = 0; Some(()) }
            #[inline]
            fn num_set_one(&mut self) -> Option<()> { *self = 1; Some(()) }

            // ops
            impl_num![op2_checked Self => add, mul, sub, div, rem];
            impl_num![op1_checked Self => neg, abs];
        }

        // NonZeroI*
        impl Num for [<NonZero $p:camel>] {
            type Inner = $p;
            type OpOut = [<NonZero $p:camel>];

            // base
            #[inline]
            fn num_into(self) -> Self::Inner { [< NonZero $p:camel >]::get(self) }
            #[inline]
            fn num_from(from: Self::Inner) -> Option<Self> { Self::new(from) }
            #[inline]
            fn num_from_ref(from: &Self::Inner) -> Option<Self> { Self::new(*from) }
            #[inline]
            fn num_set(&mut self, value: Self::Inner) -> Option<()> {
                *self = Self::new(value)?; Some(())
            }
            #[inline]
            fn num_set_ref(&mut self, value: &Self::Inner) -> Option<()> {
                *self = Self::new(*value)?; Some(())
            }

            // ident
            #[inline]
            fn num_is_zero(&self) -> Option<bool> { Some(false) }
            #[inline]
            fn num_is_one(&self) -> Option<bool> { self.get().num_is_one() }
            #[inline]
            fn num_get_zero() -> Option<Self> { None }
            #[inline]
            fn num_get_one() -> Option<Self> { Self::new(1) }
            #[inline]
            fn num_set_zero(&mut self) -> Option<()> { None }
            #[inline]
            fn num_set_one(&mut self) -> Option<()> {
                #[cfg(not(feature = "unsafe_num"))]
                { *self = Self::new(1)?; Some(()) }
                #[cfg(feature = "unsafe_num")]
                // SAFETY: we are using a constant
                { *self = unsafe { Self::new_unchecked(1) }; Some(()) }
            }

            // ops
            impl_num![op2_checked Self => mul];
            impl_num![op1_checked Self => neg, abs];
            impl_num![op2_get_checked Self => add, sub, div, rem];
        }

        // NonSpecificI*
        impl<const V: $p> Num for [<NonSpecific $p:camel>]<V> {
            type Inner = $p;
            type OpOut =  [<NonSpecific $p:camel>]<V>;
            impl_num![custom_i_body];
        }

        // NonRangeI*
        impl<const RMIN: $p, const RMAX: $p> Num for [<NonRange $p:camel>]<RMIN, RMAX> {
            type Inner = $p;
            type OpOut = [<NonRange $p:camel>]<RMIN, RMAX>;
            impl_num![custom_i_body];
        }

        // RangeI*
        impl<const RMIN: $p, const RMAX: $p> Num for [<Range $p:camel>]<RMIN, RMAX> {
            type Inner = $p;
            type OpOut = [<Range $p:camel>]<RMIN, RMAX>;
            impl_num![custom_i_body];
        }
    }};

    // Implements `Num` for unsigned integer types
    // --------------------------------------------------------------------------------------------
    (u $($p:ident),+) => { $( impl_num![@u $p]; )+ };
    (@u $p:ident) => { paste! {
        // u*
        impl Num for $p {
            type Inner = $p;
            type OpOut = $p;

            // base
            #[inline]
            fn num_into(self) -> Self::Inner { self }
            #[inline]
            fn num_from(from: Self::Inner) -> Option<Self> { Some(from) }
            #[inline]
            fn num_from_ref(from: &Self::Inner) -> Option<Self> { Some(*from) }
            #[inline]
            fn num_set(&mut self, value: Self::Inner) -> Option<()> { *self = value; Some(()) }
            #[inline]
            fn num_set_ref(&mut self, value: &Self::Inner) -> Option<()> {
                *self = *value; Some(())
            }

            // ident
            #[inline]
            fn num_is_zero(&self) -> Option<bool> { Some(*self == 0) }
            #[inline]
            fn num_is_one(&self) -> Option<bool> { Some(*self == 1) }
            #[inline]
            fn num_get_zero() -> Option<Self> { Self::num_from(0) }
            #[inline]
            fn num_get_one() -> Option<Self> { Self::num_from(1) }
            #[inline]
            fn num_set_zero(&mut self) -> Option<()> { *self = 0; Some(()) }
            #[inline]
            fn num_set_one(&mut self) -> Option<()> { *self = 1; Some(()) }

            // ops
            impl_num![op2_checked Self => add, mul, sub, div, rem];
            impl_num![op1_checked Self => neg];
            #[inline]
            fn num_abs(self) -> Option<Self> { Some(self) }
            #[inline]
            fn num_ref_abs(&self) -> Option<Self> { Some(*self) }
        }

        // NonZeroU*
        impl Num for [<NonZero $p:camel>] {
            type Inner = $p;
            type OpOut = [<NonZero $p:camel>];

            // base
            #[inline]
            fn num_into(self) -> Self::Inner { [< NonZero $p:camel >]::get(self) }
            #[inline]
            fn num_from(from: Self::Inner) -> Option<Self> { Self::new(from) }
            #[inline]
            fn num_from_ref(from: &Self::Inner) -> Option<Self> { Self::new(*from) }
            #[inline]
            fn num_set(&mut self, value: Self::Inner) -> Option<()> {
                *self = Self::new(value)?; Some(())
            }
            #[inline]
            fn num_set_ref(&mut self, value: &Self::Inner) -> Option<()> {
                *self = Self::new(*value)?; Some(())
            }

            // ident
            #[inline]
            fn num_is_zero(&self) -> Option<bool> { Some(false) }
            #[inline]
            fn num_is_one(&self) -> Option<bool> { Some(self.get() == 1) }
            #[inline]
            fn num_get_zero() -> Option<Self> { None }
            #[inline]
            fn num_get_one() -> Option<Self> { Self::new(1) }
            #[inline]
            fn num_set_zero(&mut self) -> Option<()> { None }
            #[inline]
            fn num_set_one(&mut self) -> Option<()> {
                #[cfg(not(feature = "unsafe_num"))]
                { *self = Self::new(1)?; Some(()) }
                #[cfg(feature = "unsafe_num")]
                // SAFETY: we are using a constant
                { *self = unsafe { Self::new_unchecked(1) }; Some(()) }
            }

            // ops
            impl_num![op2_checked Self => mul]; // add takes an u8 so goes below
            impl_num![op2_get_checked Self => add, sub, div, rem];
            impl_num![op1_none Self => neg]; // no neg for NonZeroU*
            #[inline]
            fn num_abs(self) -> Option<Self> { Some(self) }
            #[inline]
            fn num_ref_abs(&self) -> Option<Self> { Some(*self) }
        }

        // NonSpecificU*
        impl<const V: $p> Num for [<NonSpecific $p:camel>]<V> {
            type Inner = $p;
            type OpOut = [<NonSpecific $p:camel>]<V>;
            impl_num![custom_u_body]; }

        // NonRangeU*
        impl<const RMIN: $p, const RMAX: $p> Num for [<NonRange $p:camel>]<RMIN, RMAX> {
            type Inner = $p;
            type OpOut = [<NonRange $p:camel>]<RMIN, RMAX>;
            impl_num![custom_u_body];
        }

        // RangeU*
        impl<const RMIN: $p, const RMAX: $p> Num for [<Range $p:camel>]<RMIN, RMAX> {
            type Inner = $p;
            type OpOut = [<Range $p:camel>]<RMIN, RMAX>;
            impl_num![custom_u_body]; }
    }};

    // Implements `Num` for the floating-point types
    // --------------------------------------------------------------------------------------------
    (f $($p:ident),+) => { $( impl_num![@f $p]; )+ };
    (@f $p:ident) => {
        // f*
        impl Num for $p { paste! {
            type Inner = $p;
            type OpOut = $p;

            // base
            #[inline]
            fn num_into(self) -> Self::Inner { self }
            #[inline]
            fn num_from(from: Self::Inner) -> Option<Self> { Some(from) }
            #[inline]
            fn num_from_ref(from: &Self::Inner) -> Option<Self> { Some(*from) }
            #[inline]
            fn num_set(&mut self, value: Self::Inner) -> Option<()> { *self = value; Some(()) }
            #[inline]
            fn num_set_ref(&mut self, value: &Self::Inner) -> Option<()> {
                *self = *value; Some(())
            }

            // ident
            #[doc = "This implementation has a tolerance of 5 × [`EPSILON`][core::" $p "::EPSILON]"]
            #[inline]
            fn num_is_zero(&self) -> Option<bool> {
                Some(self.num_ref_abs()? < 5.0 * core::$p::EPSILON)
            }
            #[doc = "This implementation has a tolerance of 5 × [`EPSILON`][core::" $p "::EPSILON]"]
            #[inline]
            fn num_is_one(&self) -> Option<bool> {
                Some(self.num_sub(1.0)?.num_ref_abs()? < 5.0 * core::$p::EPSILON)
            }
            #[inline]
            fn num_get_zero() -> Option<Self> { Self::num_from(0.0) }
            #[inline]
            fn num_get_one() -> Option<Self> { Self::num_from(1.0) }
            #[inline]
            fn num_set_zero(&mut self) -> Option<()> { *self = Self::num_from(0.0)?; Some(()) }
            #[inline]
            fn num_set_one(&mut self) -> Option<()> { *self = Self::num_from(1.0)?; Some(()) }

            // ops
            impl_num![op2_float Self => add, mul, sub, div, rem];
            impl_num![op1_float Self => neg];

            #[inline]
            fn num_abs(self) -> Option<Self> {
                #[cfg(feature = "std")]
                return Some($p::abs(self));
                #[cfg(not(feature = "std"))]
                Some(iif![self >= 0.0; self; -self])
            }
            #[inline]
            fn num_ref_abs(&self) -> Option<Self> {
                #[cfg(feature = "std")]
                return Some($p::abs(*self));
                #[cfg(not(feature = "std"))]
                Some(iif![*self >= 0.0; *self; -*self])
            }
        }}
    };

    // Inner helpers for the identical body of NonSpecific, NonRange, Range
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
        fn num_abs(self) -> Option<Self> { Some(self) }
        #[inline]
        fn num_ref_abs(&self) -> Option<Self> { Some(*self) }
    };
    (custom_body) => {
        // base
        #[inline]
        fn num_into(self) -> Self::Inner { self.get() }
        #[inline]
        fn num_from(from: Self::Inner) -> Option<Self> { Self::new(from) }
        #[inline]
        fn num_from_ref(from: &Self::Inner) -> Option<Self> { Self::new(*from) }
        #[inline]
        fn num_set(&mut self, value: Self::Inner) -> Option<()> {
            *self = Self::num_from(value)?; Some(())
        }
        #[inline]
        fn num_set_ref(&mut self, value: &Self::Inner) -> Option<()> {
            *self = Self::num_from(*value)?; Some(())
        }

        // ident
        #[inline]
        fn num_is_zero(&self) -> Option<bool> { Some(self.get() == 0) }
        #[inline]
        fn num_is_one(&self) -> Option<bool> { Some(self.get() == 1) }
        #[inline]
        fn num_get_zero() -> Option<Self> { Self::num_from(0) }
        #[inline]
        fn num_get_one() -> Option<Self> { Self::num_from(1) }
        #[inline]
        fn num_set_zero(&mut self) -> Option<()> { *self = Self::num_from(0)?; Some(()) }
        #[inline]
        fn num_set_one(&mut self) -> Option<()> { *self = Self::num_from(1)?; Some(()) }
    };

    // Inner helpers for unary and binary ops
    // ============================================================================================

    /* ops that returns `None` */

    // (this could be regarded as unnecessary since it's the same as the default implementantion,
    // but it allows us to debug missing implementations while swithing the commented out blocks
    // in the num module that provides non-automatic implementations for the trait methods)
    (op1_none $Self:ty => $($op:ident),+) => {
        $( impl_num![@op1_none $Self => $op]; )+ };
    (@op1_none $Self:ty => $op:ident) => { paste! {
        #[inline]
        fn [<num_ $op>](self) -> Option<$Self::OpOut> { None }
        #[inline]
        fn [<num_ref_ $op>](&self) -> Option<$Self::OpOut> { None }
    }};
    (op2_none $Self:ty => $($op:ident),+) => {
        $( impl_num![@op2_none $Self => $op]; )+ };
    (@op2_none $Self:ty => $op:ident) => {
        #[inline]
        fn [<num_ $op>](self, other: $Self) -> Option<$Self::OpOut> { None }
        #[inline]
        fn [<num_ $op _ref>](self, other: &$Self) -> Option<$Self::OpOut> { None }
        #[inline]
        fn [<num_ref_ $op>](&self, other: $Self) -> Option<$Self::OpOut> { None }
        #[inline]
        fn [<num_ref_ $op _ref>](&self, other: &$Self) -> Option<$Self::OpOut> { None }
    };

    /* ops that call .checked() for i*, u*, and few for NonZero* */

    (op1_checked $Self:ty => $($op:ident),+) => {
        $( impl_num![@op1_checked $Self => $op]; )+ };
    (@op1_checked $Self:ty => $op:ident) => { paste! {
        #[inline]
        fn [<num_ $op>](self) -> Option<$Self::OpOut> { self.[<checked_$op>]() }
        #[inline]
        fn [<num_ref_ $op>](&self) -> Option<$Self::OpOut> { self.[<checked_$op>]() }
    }};
    (op2_checked $Self:ty => $($op:ident),+) => {
        $( impl_num![@op2_checked $Self => $op]; )+ };
    (@op2_checked $Self:ty => $op:ident) => { paste! {
        #[inline]
        fn [<num_ $op>](self, other: $Self) -> Option<$Self::OpOut> { self.[<checked_ $op>](other) }
        #[inline]
        fn [<num_ $op _ref>](self, other: &$Self) -> Option<$Self::OpOut> { self.[<checked_ $op>](*other) }
        #[inline]
        fn [<num_ref_ $op>](&self, other: $Self) -> Option<$Self::OpOut> { self.[<checked_ $op>](other) }
        #[inline]
        fn [<num_ref_ $op _ref>](&self, other: &$Self) -> Option<$Self::OpOut> {
            self.[<checked_ $op>](*other)
        }
    }};

    /* ops that call .get().checked() for: NonZero*, NonSpecific*, (Non)Range* */

    (op1_get_checked $Self:ty => $($op:ident),+) => {
        $( impl_num![@op1_get_checked $Self => $op]; )+ };
    (@op1_get_checked $Self:ty => $op:ident) => { paste! {
        #[inline]
        fn [<num_ $op>](self) -> Option<$Self::OpOut> { $Self::new(self.get().[<checked_ $op>]()?) }
        #[inline]
        fn [<num_ref_ $op>](&self) -> Option<$Self::OpOut> { $Self::new(self.get().[<checked_ $op>]()?) }
    }};
    (op2_get_checked $Self:ty => $($op:ident),+) => {
        $( impl_num![@op2_get_checked $Self => $op]; )+ };
    (@op2_get_checked $Self:ty => $op:ident) => { paste! {
        #[inline]
        fn [<num_ $op>](self, other: $Self) -> Option<$Self::OpOut> {
            $Self::new(self.get().[<checked_ $op>](other.get())?)
        }
        #[inline]
        fn [<num_ $op _ref>](self, other: &$Self) -> Option<$Self::OpOut> {
            $Self::new(self.get().[<checked_ $op>](other.get())?)
        }
        #[inline]
        fn [<num_ref_ $op>](&self, other: $Self) -> Option<$Self::OpOut> {
            $Self::new(self.get().[<checked_ $op>](other.get())?)
        }
        #[inline]
        fn [<num_ref_ $op _ref>](&self, other: &$Self) -> Option<$Self::OpOut> {
            $Self::new(self.get().[<checked_ $op>](other.get())?)
        }
    }};

    /* ops for floating-point f* types */

    (op1_float $Self:ty => $($op:ident),+) => { $( impl_num![@op1_float $Self => $op]; )+ };
    (@op1_float $Self:ty => $op:ident) => { paste! {
        #[inline]
        fn [<num_ $op>](self) -> Option<$Self::OpOut> { Some([<$op:camel>]::[<$op>](self)) }
        #[inline]
        fn [<num_ref_ $op>](&self) -> Option<$Self::OpOut> { Some([<$op:camel>]::[<$op>](self)) }
    }};
    (op2_float $Self:ty => $($op:ident),+) => { $( impl_num![@op2_float $Self => $op]; )+ };
    (@op2_float $Self:ty => $op:ident) => { paste! {
        #[inline]
        fn [<num_ $op>](self, other: $Self) -> Option<$Self::OpOut> {
            Some([<$op:camel>]::[<$op>](self, other))
        }
        #[inline]
        fn [<num_ $op _ref>](self, other: &$Self) -> Option<$Self::OpOut> {
            Some([<$op:camel>]::[<$op>](self, *other))
        }
        #[inline]
        fn [<num_ref_ $op>](&self, other: $Self) -> Option<$Self::OpOut> {
            Some([<$op:camel>]::[<$op>](self, other))
        }
        #[inline]
        fn [<num_ref_ $op _ref>](&self, other: &$Self) -> Option<$Self::OpOut> {
            Some([<$op:camel>]::[<$op>](self, *other))
        }
    }};

    // Inner helpers for identities
    // ============================================================================================
    // ...
}
use impl_num;
