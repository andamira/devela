// devela::num::int::divisor

#[allow(unused_imports)]
use crate::{
    _core::{fmt, hash, ops},
    compile, iif, isize_up, paste, usize_up,
};

/// Faster divisor for division and modulo operations.
///
/// # Features
/// It's implemented for the integer primitives enabled by the corresponding
/// [capability features][crate::_info::features#capability-features]:
/// [`_int_i8`][Self#impl-Divisor<i8>],
/// [`_int_i16`][Self#impl-Divisor<i16>],
/// [`_int_i32`][Self#impl-Divisor<i32>],
/// [`_int_i64`][Self#impl-Divisor<i64>],
/// [`_int_i128`][Self#impl-Divisor<i128>],
/// [`_int_isize`][Self#impl-Divisor<isize>],
/// [`_int_u8`][Self#impl-Divisor<u8>],
/// [`_int_u16`][Self#impl-Divisor<u16>],
/// [`_int_u32`][Self#impl-Divisor<u32>],
/// [`_int_u64`][Self#impl-Divisor<u64>],
/// [`_int_u128`][Self#impl-Divisor<u128>],
/// [`_int_usize`][Self#impl-Divisor<usize>].
///
/// # Derived Work
#[doc = include_str!("./MODIFICATIONS.md")]
#[must_use]
#[derive(Clone, Copy)]
pub struct Divisor<T> {
    inner: DivisorInner<T>,
}

/// Inner representation of [`Divisor`].
#[derive(Clone, Copy)]
enum DivisorInner<T> {
    Shift(T, u8),
    MultiplyShift(T, T, u8),
    MultiplyAddShift(T, T, u8),
    /// *Variant only used for signed numbers.*
    #[allow(dead_code)]
    ShiftAndNegate(T, u8),
    /// *Variant only used for signed numbers.*
    #[allow(dead_code)]
    MultiplyAddShiftNegate(T, T, u8),
}

/// Implements [`Divisor`]`<T>` for each enabled integer primitive.
macro_rules! impl_divisor {
    () => {
        impl_divisor![signed
            i8|u8|i16|u16:Y:"_int_i8", i16|u16|i32|u32:Y:"_int_i16", i32|u32|i64|u64:Y:"_int_i32",
            i64|u64|i128|u128:PW:"_int_i64", i128|u128|i128|u128:N:"_int_i128",
            isize|usize|isize_up|usize_up:Y:"_int_isize"
        ];
        impl_divisor![unsigned
            u8|u16:Y:"_int_u8", u16|u32:Y:"_int_u16", u32|u64:Y:"_int_u32",
            u64|u128:PW:"_int_u64", u128|u128:N:"_int_u128", usize|usize_up:Y:"_int_usize"
        ];
    };

    (
    // (un)signed entry arms
    //
    // # Arguments:
    // $t:     the type. E.g. i8.
    // $un:    the unsigned type of the same size. E.g. u8. (only for signed)
    // $up:    the upcasted type. E.g. i16.
    // $unup:  the unsigned upcasted type. E.g. u16. (only for signed)
    // $is_up: upcasted behavior. Y:upcasted | N:not upcasted | PW depends on pointer width == 64
    // $cap:   the capability feature that enables the current implementation.
     signed $( $t:ty | $un:ty | $up:ty | $unup:ty : $is_up:ident : $cap:literal),+) => {
        $( impl_divisor![@signed $t|$un|$up|$unup:$is_up:$cap]; )+
    };
    (unsigned $( $t:ty | $up:ty : $is_up:ident : $cap:literal),+) => {
        $( impl_divisor![@unsigned $t|$up:$is_up:$cap]; )+
    };
    (
    /* inner arms */
     @signed $t:ty | $un:ty | $up:ty | $unup:ty : $is_up:ident : $cap:literal) => {
        #[cfg(feature = $cap )]
        impl_divisor![@traits $t];

        #[cfg(feature = $cap )]
        #[doc = crate::doc_availability!(feature = $cap)]
        // #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = $cap)))]
        impl Divisor<$t> {
            impl_divisor![@shared $t|$un|$up|$unup:$is_up:$cap]; // shared methods

            /// Returns the absolute value of the signed primitive as its unsigned equivalent.
            #[must_use]
            const fn abs(n: $t) -> $un {
                iif![n < 0; ((-1i8) as $un).wrapping_mul(n as $un); n as $un]
            }

            /// Creates a divisor which can be used for faster computation
            /// of division and modulo by `d`.
            ///
            /// Returns `None` if `d` equals zero.
            ///
            /// # Examples
            /// ```
            /// # use devela::Divisor;
            #[doc = concat!["let d = Divisor::<", stringify![$t], ">::new(-21).unwrap();"]]
            /// ```
            #[must_use]
            pub const fn new(d: $t) -> Option<Divisor<$t>> {
                if d == 0 {
                    Self::cold_0_divisor()
                } else {
                    let ud = Self::abs(d);
                    let shift = ud.ilog2() as u8;
                    let inner = if ud.is_power_of_two() {
                        iif![d > 0; DivisorInner::Shift(d, shift); DivisorInner::ShiftAndNegate(d, shift)]
                    } else {
                        let (mut magic, rem) = Self::div_rem_wide_by_base(1 << (shift - 1), ud);
                        let e = ud - rem;
                        if e < 1 << shift {
                            DivisorInner::MultiplyShift(d, d.signum() * (magic as $t + 1), shift - 1)
                        } else {
                            magic *= 2;
                            let (doubled_rem, overflowed) = rem.overflowing_mul(2);
                            iif![doubled_rem >= ud || overflowed; magic += 1];
                            magic += 1;
                            if d > 0 {
                                DivisorInner::MultiplyAddShift(d, magic as $t, shift)
                            } else {
                                DivisorInner::MultiplyAddShiftNegate(d, -(magic as $t), shift)
                            }
                        }
                    };

                    Some(Self { inner })
                }
            }

            /// Returns the value that was used to construct this divisor as a primitive type.
            ///
            /// # Examples
            /// ```
            /// # use devela::Divisor;
            #[doc = concat!["let d = Divisor::<", stringify![$t], ">::new(-15).unwrap();"]]
            /// assert_eq!(d.get(), -15);
            /// ```
            #[must_use]
            pub const fn get(&self) -> $t {
                match self.inner {
                    DivisorInner::Shift(d, _) => d,
                    DivisorInner::ShiftAndNegate(d, _) => d,
                    DivisorInner::MultiplyShift(d, _, _) => d,
                    DivisorInner::MultiplyAddShift(d, _, _) => d,
                    DivisorInner::MultiplyAddShiftNegate(d, _, _) => d,
                }
            }

            /// Returns `true` if `n` is divisible by `self`.
            ///
            /// We take `0` to be divisible by all non-zero numbers.
            ///
            /// # Examples
            /// ```
            /// # use devela::Divisor;
            #[doc = concat!["let d = Divisor::<", stringify![$t], ">::new(-9).unwrap();"]]
            /// assert!(d.divides(27));
            /// ```
            #[must_use]
            pub const fn divides(&self, n: $t) -> bool {
                self.rem_of(n) == 0
            }

            /// Returns the remainder of dividing `n` by `self`.
            ///
            /// # Examples
            /// ```
            /// # use devela::Divisor;
            #[doc = concat!["let d = Divisor::<", stringify![$t], ">::new(21).unwrap();"]]
            /// let rem = d.rem_of(-30);
            /// assert_eq!(rem, -9);
            /// ```
            #[must_use]
            pub const fn rem_of(&self, n: $t) -> $t {
                n.wrapping_add((self.get().wrapping_mul(self.div_of(n))).wrapping_mul(-1))
            }

            /// Returns the result of dividing `n` by `self`.
            ///
            /// This will perform a wrapping division, i.e.
            #[doc = concat!("`Divisor::<", stringify!($t), ">::new(-1).unwrap().div_of(",
                stringify!($t) ,"::MIN)`")]
            /// will always silently return
            #[doc = concat!("`", stringify!($t) ,"::MIN`")]
            /// whether the program was compiled with `overflow-checks` turned off or not.
            ///
            /// # Examples
            /// ```
            /// # use devela::Divisor;
            #[doc = concat!["let d = Divisor::<", stringify![$t], ">::new(13).unwrap();"]]
            /// let div = d.div_of(-30);
            /// assert_eq!(div, -2);
            /// ```
            #[must_use]
            pub const fn div_of(&self, n: $t) -> $t {
                match self.inner {
                    DivisorInner::Shift(_, shift) => {
                        let mask = (1 as $t << shift).wrapping_sub(1);
                        let b = (n >> (<$t>::BITS - 1)) & mask;
                        n.wrapping_add(b) >> shift
                    },
                    DivisorInner::ShiftAndNegate(_, shift) => {
                        let mask = (1 as $t << shift).wrapping_sub(1);
                        let b = (n >> (<$t>::BITS - 1)) & mask;
                        let t = n.wrapping_add(b) >> shift;
                        t.wrapping_mul(-1)
                    },
                    DivisorInner::MultiplyShift(_, magic, shift) => {
                        let q = Self::mulh(magic, n) >> shift;
                        iif![q < 0; q + 1; q]
                    },
                    DivisorInner::MultiplyAddShift(_, magic, shift) => {
                        let q = Self::mulh(magic, n);
                        let t = q.wrapping_add(n) >> shift;
                        iif![t < 0; t + 1; t]
                    },
                    DivisorInner::MultiplyAddShiftNegate(_, magic, shift) => {
                        let q = Self::mulh(magic, n);
                        let t = q.wrapping_add(n.wrapping_mul(-1)) >> shift;
                        iif![t < 0; t + 1; t]
                    }
                }
            }
        }
    };

    (@unsigned $t:ty | $up:ty : $is_up:ident : $cap:literal) => {
        #[cfg(feature = $cap )]
        impl_divisor![@traits $t];

        #[cfg(feature = $cap )]
        #[doc = crate::doc_availability!(feature = $cap)]
        // #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = $cap)))]
        impl Divisor<$t> {
            impl_divisor![@shared $t|$t|$up|$up:$is_up:$cap]; // shared methods

            /// Creates a divisor which can be used for faster computation
            /// of division and modulo by `d`.
            ///
            /// Returns `None` if `d` equals zero.
            ///
            /// # Examples
            /// ```
            /// # use devela::Divisor;
            #[doc = concat!["let _d = Divisor::<", stringify![$t], ">::new(5);"]]
            /// ```
            #[must_use]
            pub const fn new(d: $t) -> Option<Divisor<$t>> {
                if d == 0 {
                    Self::cold_0_divisor()
                } else {
                    let shift = d.ilog2() as u8;
                    let inner = if d.is_power_of_two() {
                        DivisorInner::Shift(d, shift)
                    } else {
                        let (mut magic, rem) = Self::div_rem_wide_by_base(1 << shift, d);
                        let e = d - rem;
                        if e < 1 << shift {
                            DivisorInner::MultiplyShift(d, magic + 1, shift)
                        } else {
                            magic = magic.wrapping_mul(2);
                            let (doubled_rem, overflowed) = rem.overflowing_mul(2);
                            if doubled_rem >= d || overflowed { magic += 1; }
                            DivisorInner::MultiplyAddShift(d, magic + 1, shift)
                        }
                    };
                    Some(Self { inner })
                }
            }

            /// Returns the value that was used to construct this divisor as a primitive type.
            ///
            /// # Examples
            /// ```
            /// # use devela::Divisor;
            #[doc = concat!["let d = Divisor::<", stringify![$t], ">::new(7).unwrap();"]]
            /// assert_eq!(d.get(), 7);
            /// ```
            #[must_use]
            pub const fn get(&self) -> $t {
                match self.inner {
                    DivisorInner::Shift(d, _) => d,
                    DivisorInner::MultiplyShift(d, _, _) => d,
                    DivisorInner::MultiplyAddShift(d, _, _) => d,
                    _ => unreachable![],
                }
            }

            /// Returns `true` if `n` is divisible by `self`.
            ///
            /// We take `0` to be divisible by all non-zero numbers.
            ///
            /// # Examples
            /// ```
            /// # use devela::Divisor;
            #[doc = concat!["let d = Divisor::<", stringify![$t], ">::new(17).unwrap();"]]
            /// assert!(d.divides(34));
            /// ```
            #[must_use]
            pub const fn divides(&self, n: $t) -> bool {
                self.rem_of(n) == 0
            }

            /// Returns the remainder of dividing `n` by `self`.
            ///
            /// # Examples
            /// ```
            /// # use devela::Divisor;
            #[doc = concat!["let d = Divisor::<", stringify![$t], ">::new(11).unwrap();"]]
            /// let rem = d.rem_of(30);
            /// assert_eq!(rem, 8);
            /// ```
            #[must_use]
            pub const fn rem_of(&self, n: $t) -> $t {
                n - self.get() * self.div_of(n)
            }

            /// Returns the result of dividing `n` by `self`.
            ///
            /// # Examples
            /// ```
            /// # use devela::Divisor;
            #[doc = concat!["let d = Divisor::<", stringify![$t], ">::new(17).unwrap();"]]
            /// let div = d.div_of(34);
            /// assert_eq!(div, 2);
            /// ```
            #[must_use]
            pub const fn div_of(&self, n: $t) -> $t {
                match self.inner {
                    DivisorInner::Shift(_, shift) => n >> shift,
                    DivisorInner::MultiplyShift(_, magic, shift) => Self::mulh(magic, n) >> shift,
                    DivisorInner::MultiplyAddShift(_, magic, shift) => {
                        let q = Self::mulh(magic, n);
                        let t = ((n - q) >> 1) + q;
                        t >> shift
                    },
                    _ => unreachable![], // the remaining arms are only for signed
                }
            }
        }
    };

    (@shared $t:ty | $un:ty | $up:ty | $unup:ty : $is_up:ident : $cap:literal) => {
        paste!{
            /// Alias of [`new`][Self::new] with a unique name that helps type inference.
            pub const fn [<new_ $t>](d: $t) -> Option<Divisor<$t>> { Self::new(d) }
        }

        /// Helper function to be called from the cold path branch when divisor == 0.
        #[cold] #[inline(never)]
        const fn cold_0_divisor() -> Option<Self> { None }

        /// Multiply two words together, returning only the top half of the product.
        ///
        /// Works by extending the factors to 2N-bits, using the built-in 2N-by-2N-bit
        /// multiplication and shifting right to the top half only.
        #[compile(any(same($is_up, Y), all(same($is_up, PW), pointer_width_eq(64))))]
        const fn mulh(x: $t, y: $t) -> $t {
            (((x as $up) * (y as $up)) >> <$t>::BITS) as $t
        }
        /// Non-upcasting version, adapted from Figure 8-2 in Hacker's Delight, 2nd Ed.
        #[compile(any(same($is_up, N), all(same($is_up, PW), not(pointer_width_eq(64)))))]
        const fn mulh(x: $t, y: $t) -> $t {
            const HALF_WIDTH_BITS: u32 = <$t>::BITS / 2;
            const LOWER_HALF_MASK: $t = (1 << HALF_WIDTH_BITS) - 1;

            let x_low = x & LOWER_HALF_MASK;
            let y_low = y & LOWER_HALF_MASK;
            let t = x_low.wrapping_mul(y_low);
            let k = t >> HALF_WIDTH_BITS;

            let x_high = x >> HALF_WIDTH_BITS;
            let t = x_high.wrapping_mul(y_low) + k;
            let k = t & LOWER_HALF_MASK;
            let w1 = t >> HALF_WIDTH_BITS;

            let y_high = y >> HALF_WIDTH_BITS;
            let t = x_low.wrapping_mul(y_high) + k;
            let k = t >> HALF_WIDTH_BITS;

            x_high.wrapping_mul(y_high) + w1 + k
        }

        /// Divide a 2N-bit dividend by an N-bit divisor with remainder, assuming
        /// that the result fits into N bits and that the lower half of bits of the
        /// dividend are all zero.
        ///
        /// Works by extending the dividend to 2N-bits and then using the built-in
        /// 2N-by-2N-bit division method.
        #[compile(any(same($is_up, Y), all(same($is_up, PW), pointer_width_eq(64))))]
        const fn div_rem_wide_by_base(top_half: $un, d: $un) -> ($un, $un) {
            let n = (top_half as $unup) << <$un>::BITS;
            let quot = (n / (d as $unup)) as $un;
            let rem = (n % (d as $unup)) as $un;
            (quot, rem)
        }
        /// Non-upcasting version, adapted from Figure 9-3 in Hacker's Delight, 2nd Ed.
        #[compile(any(same($is_up, N), all(same($is_up, PW), not(pointer_width_eq(64)))))]
        const fn div_rem_wide_by_base(top_half: $un, d: $un) -> ($un, $un) {
            const HALF_WORD_BITS: u32 = <$un>::BITS / 2;
            const BASE: $un = 1 << HALF_WORD_BITS;
            let s = d.leading_zeros();
            let v = d << s;
            let vn1 = v >> HALF_WORD_BITS;
            let vn0 = v & (BASE - 1);
            let un32 = top_half << s;
            let mut q1 = un32 / vn1;
            let mut rhat = un32 - q1 * vn1;
            loop {
                if q1 >= BASE || q1 * vn0 > (rhat << HALF_WORD_BITS) {
                    q1 -= 1;
                    rhat += vn1;
                    iif![rhat < BASE; continue];
                }
                break;
            }
            let un21 = (un32 << HALF_WORD_BITS).wrapping_sub(q1.wrapping_mul(v));
            let mut q0 = un21 / vn1;
            rhat = un21 - q0 * vn1;
            loop {
                if q0 >= BASE || q0 * vn0 > (rhat << HALF_WORD_BITS) {
                    q0 -= 1;
                    rhat += vn1;
                    iif![rhat < BASE; continue];
                }
                break;
            }
            let r = ((un21 << HALF_WORD_BITS).wrapping_sub(q0.wrapping_mul(v))) >> s;
            ((q1 << HALF_WORD_BITS) + q0, r)
        }
    };

    (@traits $t:ty) => {
        impl PartialEq for Divisor<$t> {
            fn eq(&self, other: &Self) -> bool { self.get() == other.get() }
        }
        impl Eq for Divisor<$t> {}
        impl fmt::Debug for Divisor<$t> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "{}", self.get()) }
        }
        impl fmt::Display for Divisor<$t> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "{}", self.get()) }
        }
        impl hash::Hash for Divisor<$t> {
            fn hash<H: hash::Hasher>(&self, state: &mut H) { self.get().hash(state); }
        }
        impl ops::Div<Divisor<$t>> for $t {
            type Output = $t;
            fn div(self, rhs: Divisor<$t>) -> Self::Output { rhs.div_of(self) }
        }
        impl ops::DivAssign<Divisor<$t>> for $t {
            fn div_assign(&mut self, rhs: Divisor<$t>) { *self = rhs.div_of(*self) }
        }
        impl ops::Rem<Divisor<$t>> for $t {
            type Output = $t;
            fn rem(self, rhs: Divisor<$t>) -> Self::Output { rhs.rem_of(self) }
        }
        impl ops::RemAssign<Divisor<$t>> for $t {
            fn rem_assign(&mut self, rhs: Divisor<$t>) { *self = rhs.rem_of(*self) }
        }
    };
}
impl_divisor!();
