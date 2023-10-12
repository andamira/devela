// devela::num::trait
//
//!
//

use super::*;
use crate::codegen::{iif, paste};
use core::ops::{Add, Div, Mul, Neg, Sub};

/// Common trait for numeric types.
///
/// Any concrete implementation must manually implement the operations it wants
/// to support, otherwise they will return `None`.
///
/// Binary operations offer two alternative methods, one for when you want to
/// transfer ownership of the second element, and another one for when you don't.
/// Transferring ownership is more efficient for `Copy` types, and using a
/// reference is more appropriate for non-copy types.
///
/// For the default implementations we try to always offer a meaningful result,
/// even if the concrete type doesn't support it directly, we do the operation
/// on the underlying primitive and try to construct the new type again.
/// If there's an error or an impossibility at any point it just returns `None`.
///
/// The standard library offers different methods for signed and unsigned types,
/// (e.g. abs, neg), and some are lacking for non-zero types (div, sub).
/// This trait try to offer the same methods everywhere and give a
/// result when a result is possible.
#[rustfmt::skip]
#[allow(unused_variables)]
pub trait Num {
    /// The primitive type associated to this numeric type.
    type Primitive;

    /// Returns the value in the form of the associated `Primitive` type.
    #[must_use]
    fn num_into(&self) -> Self::Primitive;

    /// Returns `Self` when given a valid `value`.
    #[must_use]
    fn num_from(value: Self::Primitive) -> Option<Self> where Self: Sized;

    /* Operations */

    /// Computes `self` + `other` for `Copy` types.
    #[must_use]
    fn num_add(&self, other: Self) -> Option<Self> where Self: Sized + Copy { None }
    /// Computes `self` + `other` for non-copy types.
    #[must_use]
    fn num_add_ref(&self, other: &Self) -> Option<Self> where Self: Sized { None }
    /// Computes `self` - `other` for `Copy` types.
    #[must_use]
    fn num_sub(&self, other: Self) -> Option<Self> where Self: Sized + Copy { None }
    /// Computes `self` - `other` for non-copy types.
    #[must_use]
    fn num_sub_ref(&self, other: &Self) -> Option<Self> where Self: Sized { None }
    /// Computes `self` * `other` for `Copy` types.
    #[must_use]
    fn num_mul(&self, other: Self) -> Option<Self> where Self: Sized + Copy { None }
    /// Computes `self` * `other` for non-copy types.
    #[must_use]
    fn num_mul_ref(&self, other: &Self) -> Option<Self> where Self: Sized { None }
    /// Computes `self` / `other` for `Copy` types.
    #[must_use]
    fn num_div(&self, other: Self) -> Option<Self> where Self: Sized + Copy { None }
    /// Computes `self` / `other` for `Copy` types.
    #[must_use]
    fn num_div_ref(&self, other: &Self) -> Option<Self> where Self: Sized { None }

    /// Computes `-self`.
    #[must_use]
    fn num_neg(&self) -> Option<Self> where Self: Sized { None }

    /// Computes the absolute value of `self`.
    #[must_use]
    fn num_abs(&self) -> Option<Self> where Self: Sized { None }
}

macro_rules! impl_num {
    [] => {
        impl_num![i i8, i16, i32, i64, i128, isize];
        impl_num![u u8, u16, u32, u64, u128, usize];
        impl_num![f f32, f64];
    };

    // Implements `Num` for signed integer types
    (i $($p:ident),+) => { $( impl_num![@i $p]; )+ };
    (@i $p:ident) => { paste! {
        // i*
        impl Num for $p {
            type Primitive = $p;
            #[inline]
            fn num_into(&self) -> Self::Primitive { *self }
            #[inline]
            fn num_from(from: Self::Primitive) -> Option<Self> { Some(from) }

            #[inline]
            fn num_add(&self, other: Self) -> Option<Self> { self.checked_add(other) }
            #[inline]
            fn num_add_ref(&self, other: &$p) -> Option<Self> { self.checked_add(*other) }
            #[inline]
            fn num_sub(&self, other: Self) -> Option<Self> { self.checked_sub(other) }
            #[inline]
            fn num_sub_ref(&self, other: &$p) -> Option<Self> { self.checked_sub(*other) }
            #[inline]
            fn num_mul(&self, other: Self) -> Option<Self> { self.checked_mul(other) }
            #[inline]
            fn num_mul_ref(&self, other: &$p) -> Option<Self> { self.checked_mul(*other) }
            #[inline]
            fn num_div(&self, other: Self) -> Option<Self> { self.checked_div(other) }
            #[inline]
            fn num_div_ref(&self, other: &$p) -> Option<Self> { self.checked_div(*other) }

            #[inline]
            fn num_neg(&self) -> Option<Self> { self.checked_neg() }
            #[inline]
            fn num_abs(&self) -> Option<Self> { self.checked_abs() }
        }

        // NonZeroI*
        impl Num for [<NonZero $p:camel>] {
            type Primitive = $p;
            #[inline]
            fn num_into(&self) -> Self::Primitive { [< NonZero $p:camel >]::get(*self) }
            #[inline]
            fn num_from(from: Self::Primitive) -> Option<Self> { Self::new(from) }

            // supported in core
            #[inline]
            fn num_mul(&self, other: Self) -> Option<Self> { self.checked_mul(other) }
            #[inline]
            fn num_mul_ref(&self, other: &Self) -> Option<Self> { self.checked_mul(*other) }
            #[inline]
            fn num_neg(&self) -> Option<Self> { self.checked_neg() }

            impl_num![op2 add];
            impl_num![op2 sub];
            impl_num![op2 div];
            impl_num![op1 abs];
        }

        // NonSpecificI*
        impl<const V: $p> Num for [<NonSpecific $p:camel>]<V> {
            type Primitive = $p;
            #[inline]
            fn num_into(&self) -> Self::Primitive { self.get() }
            #[inline]
            fn num_from(from: Self::Primitive) -> Option<Self> { Self::new(from) }

            impl_num![op2 add];
            impl_num![op2 mul];
            impl_num![op2 sub];
            impl_num![op2 div];
            impl_num![op1 neg];
            impl_num![op1 abs];
        }

        // NonRangeI*
        impl<const RMIN: $p, const RMAX: $p> Num for [<NonRange $p:camel>]<RMIN, RMAX> {
            type Primitive = $p;
            #[inline]
            fn num_into(&self) -> Self::Primitive { self.get() }
            #[inline]
            fn num_from(from: Self::Primitive) -> Option<Self> { Self::new(from) }

            impl_num![op2 add];
            impl_num![op2 mul];
            impl_num![op2 sub];
            impl_num![op2 div];
            impl_num![op1 neg];
            impl_num![op1 abs];
        }

        // RangeI*
        impl<const RMIN: $p, const RMAX: $p> Num for [<Range $p:camel>]<RMIN, RMAX> {
            type Primitive = $p;
            #[inline]
            fn num_into(&self) -> Self::Primitive { self.get() }
            #[inline]
            fn num_from(from: Self::Primitive) -> Option<Self> { Self::new(from) }

            impl_num![op2 add];
            impl_num![op2 mul];
            impl_num![op2 sub];
            impl_num![op2 div];
            impl_num![op1 neg];
            impl_num![op1 abs];
        }
    }};

    // Implements `Num` for unsigned integer types
    (u $($p:ident),+) => { $( impl_num![@u $p]; )+ };
    (@u $p:ident) => { paste! {
        // u*
        impl Num for $p {
            type Primitive = $p;
            #[inline]
            fn num_into(&self) -> Self::Primitive { *self }
            #[inline]
            fn num_from(from: Self::Primitive) -> Option<Self> { Some(from) }

            #[inline]
            fn num_add(&self, other: Self) -> Option<Self> { self.checked_add(other) }
            #[inline]
            fn num_add_ref(&self, other: &$p) -> Option<Self> { self.checked_add(*other) }
            #[inline]
            fn num_sub(&self, other: Self) -> Option<Self> { self.checked_sub(other) }
            #[inline]
            fn num_sub_ref(&self, other: &$p) -> Option<Self> { self.checked_sub(*other) }
            #[inline]
            fn num_mul(&self, other: Self) -> Option<Self> { self.checked_mul(other) }
            #[inline]
            fn num_mul_ref(&self, other: &$p) -> Option<Self> { self.checked_mul(*other) }
            #[inline]
            fn num_div(&self, other: Self) -> Option<Self> { self.checked_div(other) }
            #[inline]
            fn num_div_ref(&self, other: &$p) -> Option<Self> { self.checked_div(*other) }

            #[inline]
            fn num_neg(&self) -> Option<Self> { self.checked_neg() }
            #[inline]
            fn num_abs(&self) -> Option<Self> { Some(*self) }
        }

        // NonZeroU*
        impl Num for [<NonZero $p:camel>] {
            type Primitive = $p;
            #[inline]
            fn num_into(&self) -> Self::Primitive { [< NonZero $p:camel >]::get(*self) }
            #[inline]
            fn num_from(from: Self::Primitive) -> Option<Self> { Self::new(from) }

            // supported in core
            #[inline]
            fn num_add(&self, other: Self) -> Option<Self> { self.checked_add(other.get()) }
            #[inline]
            fn num_add_ref(&self, other: &Self) -> Option<Self> { self.checked_add(other.get()) }
            #[inline]
            fn num_mul(&self, other: Self) -> Option<Self> { self.checked_mul(other) }
            #[inline]
            fn num_mul_ref(&self, other: &Self) -> Option<Self> { self.checked_mul(*other) }

            impl_num![op2 sub];
            impl_num![op2 div];
            // neg is always None

            #[inline]
            fn num_abs(&self) -> Option<Self> { Some(*self) }
        }

        // NonSpecificU*
        impl<const V: $p> Num for [<NonSpecific $p:camel>]<V> {
            type Primitive = $p;
            #[inline]
            fn num_into(&self) -> Self::Primitive { self.get() }
            #[inline]
            fn num_from(from: Self::Primitive) -> Option<Self> { Self::new(from) }

            impl_num![op2 add];
            impl_num![op2 mul];
            impl_num![op2 sub];
            impl_num![op2 div];
            impl_num![op1 neg];

            #[inline]
            fn num_abs(&self) -> Option<Self> { Some(*self) }
        }

        // NonRangeU*
        impl<const RMIN: $p, const RMAX: $p> Num for [<NonRange $p:camel>]<RMIN, RMAX> {
            type Primitive = $p;
            #[inline]
            fn num_into(&self) -> Self::Primitive { self.get() }
            #[inline]
            fn num_from(from: Self::Primitive) -> Option<Self> { Self::new(from) }

            impl_num![op2 add];
            impl_num![op2 mul];
            impl_num![op2 sub];
            impl_num![op2 div];
            impl_num![op1 neg];

            #[inline]
            fn num_abs(&self) -> Option<Self> { Some(*self) }
        }

        impl<const RMIN: $p, const RMAX: $p> Num for [<Range $p:camel>]<RMIN, RMAX> {
            type Primitive = $p;
            #[inline]
            fn num_into(&self) -> Self::Primitive { self.get() }
            #[inline]
            fn num_from(from: Self::Primitive) -> Option<Self> { Self::new(from) }

            impl_num![op2 add];
            impl_num![op2 mul];
            impl_num![op2 sub];
            impl_num![op2 div];
            impl_num![op1 neg];

            #[inline]
            fn num_abs(&self) -> Option<Self> { Some(*self) }
        }
    }};

    // Implements `Num` for the floating-point types
    (f $($p:ident),+) => { $( impl_num![@f $p]; )+ };
    (@f $p:ident) => {
        // f*
        impl Num for $p {
            type Primitive = $p;
            #[inline]
            fn num_into(&self) -> Self::Primitive { *self }
            #[inline]
            fn num_from(from: Self::Primitive) -> Option<Self> { Some(from) }

            #[inline]
            fn num_add(&self, other: Self) -> Option<Self> { Some(Add::add(self, other)) }
            #[inline]
            fn num_add_ref(&self, other: &$p) -> Option<Self> { Some(Add::add(self, other)) }
            #[inline]
            fn num_sub(&self, other: Self) -> Option<Self> { Some(Sub::sub(self, other)) }
            #[inline]
            fn num_sub_ref(&self, other: &$p) -> Option<Self> { Some(Sub::sub(self, other)) }
            #[inline]
            fn num_mul(&self, other: Self) -> Option<Self> { Some(Mul::mul(self, other)) }
            #[inline]
            fn num_mul_ref(&self, other: &$p) -> Option<Self> { Some(Mul::mul(self, other)) }
            #[inline]
            fn num_div(&self, other: Self) -> Option<Self> { Some(Div::div(self, other)) }
            #[inline]
            fn num_div_ref(&self, other: &$p) -> Option<Self> { Some(Div::div(self, other)) }

            #[inline]
            fn num_neg(&self) -> Option<Self> { Some(Neg::neg(self)) }

            #[inline]
            fn num_abs(&self) -> Option<Self> {
                #[cfg(feature = "std")]
                return Some($p::abs(*self));
                #[cfg(not(feature = "std"))]
                Some(iif![ *self >= 0.0; *self; -*self ])
            }
        }
    };

    // Inner helpers
    //
    // They try to do the operation using the underlying primitive types,
    // and finally tries to construct a new Self with the result.

    // Implements a custom unary numeric operation,
    (op1 $op:ident) => { paste! {
        #[inline]
        fn [<num_ $op>](&self) -> Option<Self> {
            iif![let Some(n) = self.get().[<checked_ $op>](); Self::new(n); None]
        }
    }};

    // Implements a custom binary numeric operation.
    (op2 $op:ident) => { paste! {
        #[inline]
        fn [<num_ $op>](&self, other: Self) -> Option<Self> {
            iif![let Some(n) = self.get().[<checked_ $op>](other.get()); Self::new(n); None]
        }
        #[inline]
        fn [<num_ $op _ref>](&self, other: &Self) -> Option<Self> { self.[<num_ $op>](*other) }
    }};
}
impl_num![];
