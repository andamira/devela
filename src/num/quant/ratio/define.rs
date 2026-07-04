// devela/src/num/quant/ratio/define.rs
//
//! Defines [`Ratio`].
//

use crate::IntError::{
    self, MismatchedSizes, NoInverse, NonNegativeRequired, NonZeroRequired, Overflow,
};
use crate::Ordering::{self, Equal, Greater, Less};
use crate::Sign::{Negative, Positive};
use crate::{
    _impl_init, Char, Digits, FmtError, Str, cif, cold_path, compile, cswap, impl_trait, is,
    isize_up, slice, unlikely, unwrap, usize_up,
};

type RatioResult<T> = Result<T, IntError>;

crate::CONST! {
    _DOC_RATIO = "`Ratio` represents scale, density, aspect, rate, or conversion.
It stores the numerator and denominator as separate terms.";
}

#[doc = crate::_tags!(quant)]
/// A proportional relation `n / d`.
#[doc = crate::_doc_meta!{
    location("num/quant"),
    test_size_of(RatioI32 = 8; niche Option),
}]
#[doc = _DOC_RATIO!()]
///
/// The generic form does not define arithmetic semantics by itself.
/// Primitive aliases such as [`RatioI32`] and [`RatioU32`] use nonzero
/// denominators and provide checked constructors.
///
/// # Examples
/// ```
/// # use devela::RatioU8;
/// let aspect_ratio = RatioU8::new(16, 9).unwrap();
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Ratio<N, D> {
    /// The numerator.
    n: N,
    /// The denominator.
    d: D,
}

#[rustfmt::skip]
impl<N, D> Ratio<N, D> {
    /// Constructs a ratio from its stored parts.
    pub const fn from_parts(n: N, d: D) -> Self { Self { n, d } }
    /// Returns a reference to the stored numerator.
    pub const fn num_ref(&self) -> &N { &self.n }
    /// Returns a reference to the stored denominator.
    pub const fn den_ref(&self) -> &D { &self.d }
    /// Decomposes the ratio into its stored parts.
    pub fn into_parts(self) -> (N, D) { (self.n, self.d) }
}

macro_rules! _impl_ratio_prim {
    () => {
        // primitive : unsigned : upcasted : is_signed
        _impl_ratio_prim!(
            i8:u8:i16:true, i16:u16:i32:true, i32:u32:i64:true, i64:u64:i128:true,
            i128:u128:i128:true, isize:usize:isize_up:true,
            u8:u8:u16:false, u16:u16:u32:false, u32:u32:u64:false, u64:u64:u128:false,
            u128:u128:u128:false, usize:usize:usize_up:false,
        );
    };
    ($($Prim:ty : $Unsigned:ty : $Upcasted:ty : $is_signed:tt),+ $(,)?) => { $crate::paste! { $(
        _impl_ratio_prim!(% [<Ratio $Prim:camel>], $Prim, $crate::[<NonZero $Prim:camel>],
            $Unsigned, $Upcasted, $is_signed);
    )+ }};
    (% $Name:ident, $N:ty, $D:ty, $Unsigned:ty, $Upcasted:ty, $is_signed:tt) => {
        #[doc = crate::_tags!(quant)]
        #[doc = concat!("An `", stringify!($N), "` [`Ratio`] with a nonzero denominator.")]
        #[doc = crate::_doc_meta!{location("num/quant")}]
        #[doc = _DOC_RATIO!()]
        pub type $Name = Ratio<$N, $D>;

        _impl_init! { Self::ZERO => $Name }
        impl Default for $Name { fn default() -> Self { Self::ZERO } }

        impl_trait! { fmt::Display for $Name |self, f| {
            let mut buf = [0u8; Self::STR_BUF_LEN];
            let s = self.write_str_with_sep(&mut buf, ':').map_err(|_| FmtError)?;
            f.write_str(s)
        }}

        impl $Name {
            /* constants */

            /// The zero ratio `0/1`.
            pub const ZERO: Self = Self { n: 0, d: Self::_den_one() };
            /// The unit ratio `1/1`.
            pub const ONE: Self =  Self { n: 1, d: Self::_den_one() };

            #[compile(same($is_signed, true))]
            /// The negative unit ratio `-1/1`.
            pub const NEG_ONE: Self = Self { n: -1, d: Self::_den_one() };

            /// Maximum number of bytes needed to write this ratio in decimal form.
            pub const STR_BUF_LEN: usize = Digits::<$Unsigned>::MAX_DIGITS_10 as usize * 2 + 6;

            /* constructors */

            /// Constructs a ratio from primitive terms.
            ///
            /// Returns `None` if `den == 0`.
            pub const fn new(n: $N, d: $N) -> Option<Self> {
                let Some(d) = <$D>::new(d) else { cold_path(); return None; };
                Some(Self::new_nonzero(n, d))
            }
            /// Constructs a ratio from a nonzero denominator.
            pub const fn new_nonzero(n: $N, d: $D) -> Self { Self { n, d } }

            /// Constructs a ratio from primitive terms.
            ///
            /// Returns `NonZeroRequired` if `den == 0`.
            pub const fn try_new(n: $N, d: $N) -> RatioResult<Self> {
                let Some(d) = <$D>::new(d) else {
                    cold_path(); return Err(NonZeroRequired); };
                Ok(Self::new_nonzero(n, d))
            }

            /// Constructs the integer ratio `n/1`.
            pub const fn from_int(n: $N) -> Self { Self { n, d: Self::_den_one() } }

            /* queries */

            /// Returns the numerator.
            pub const fn num(self) -> $N { self.n }
            /// Returns the denominator as its primitive carrier.
            pub const fn den(self) -> $N { self.d.get() }
            /// Returns the denominator as a nonzero value.
            pub const fn den_nonzero(self) -> $D { self.d }

            /// Returns `(num, den)` as primitive carriers.
            pub const fn num_den(self) -> ($N, $N) { (self.n, self.d.get()) }

            /// Returns whether the numerator is zero.
            pub const fn is_zero(self) -> bool { self.n == 0 }
            /// Returns whether the stored terms represent one.
            pub const fn is_one(self) -> bool { self.n == self.d.get() }

            /// Returns whether the ratio represents an integer.
            pub const fn is_integer(self) -> bool { self._is_integer_checked() }

            /// Returns whether the ratio is positive.
            pub const fn is_positive(self) -> bool { self._is_positive() }
            /// Returns whether the ratio is negative.
            pub const fn is_negative(self) -> bool { self._is_negative() }

            // unsigned
            #[compile(same($is_signed, false))]
            /// Returns whether the ratio is proper.
            pub const fn is_proper(self) -> bool { self.n < self.d.get() }
            #[compile(same($is_signed, false))]
            /// Returns whether the ratio is reduced.
            pub const fn is_reduced(self) -> bool { self._gcd_unchecked() == 1 }

            // signed
            #[compile(same($is_signed, true))]
            /// Returns whether the ratio is proper.
            pub const fn is_proper(self) -> RatioResult<bool> {
                if unlikely(self._has_min_term()) { return Err(Overflow(None)); }
                let (n, d) = self._num_den_abs();
                Ok(n < d)
            }
            #[compile(same($is_signed, true))]
            /// Returns whether the ratio is reduced.
            pub const fn is_reduced(self) -> RatioResult<bool> {
                match self.gcd() { Ok(g) => Ok(g == 1), Err(e) => Err(e) }
            }

            /// Returns the relative ordering of two ratios.
            pub const fn cmp_value(self, other: Self) -> RatioResult<Ordering> {
                let Some(left) = (self.n as $Upcasted).checked_mul(other.d.get() as $Upcasted)
                    else { return Err(Overflow(None)); };
                let Some(right) = (other.n as $Upcasted).checked_mul(self.d.get() as $Upcasted)
                    else { return Err(Overflow(None)); };
                let ord = if left < right { Less } else if left > right { Greater } else { Equal };
                // if denominators have opposite signs, cross-multiplication reverses order
                if cif!(same($is_signed, true)) && (self.d.get() > 0) != (other.d.get() > 0) {
                    return Ok(ord.reverse());
                }
                Ok(ord)
            }

            /* modifications */

            /// Returns the reciprocal.
            ///
            /// Returns `NoInverse` if the numerator is zero.
            pub const fn recip(self) -> RatioResult<Self> {
                unwrap![some_ok_or Self::new(self.d.get(), self.n), NoInverse]
            }

            #[compile(same($is_signed, false))]
            /// Returns a reduced copy of the ratio.
            ///
            /// This divides both terms by their greatest common divisor.
            /// It does not force a positive denominator.
            pub const fn reduced(self) -> Self { self._reduced_unchecked() }
            #[compile(same($is_signed, true))]
            /// Returns a reduced copy of the ratio.
            ///
            /// This divides both terms by their greatest common divisor.
            /// It does not force a positive denominator.
            ///
            /// Returns `Overflow` if a signed term is the minimum value.
            pub const fn reduced(self) -> RatioResult<Self> {
                if unlikely(self._has_min_term()) { return Err(Overflow(None)); }
                Ok(self._reduced_unchecked())
            }

            // the following are only for signed:
            #[compile(same($is_signed, true))]
            /// Returns a reduced copy, or `self` if reduction overflows.
            pub const fn reduced_or_self(self) -> Self {
                match self.reduced() { Ok(r) => r, Err(_) => self }
            }
            #[compile(same($is_signed, true))]
            /// Returns a copy with a positive denominator.
            ///
            /// This moves the denominator sign to the numerator. It does not reduce the terms.
            ///
            /// Returns `Overflow` if changing the sign is not representable.
            pub const fn with_pos_den(self) -> RatioResult<Self> {
                if self.den() > 0 { return Ok(self); }
                // den is negative here. Both signs must be flipped.
                if unlikely(self.den() == <$N>::MIN || self.num() == <$N>::MIN) {
                    cold_path(); return Err(Overflow(None)); }
                let n = -self.num();
                let d = -self.den();
                let d = unwrap![some_guaranteed_or_ub <$D>::new(d)];
                Ok(Self { n, d })
            }
            #[compile(same($is_signed, true))]
            /// Returns a copy with a positive denominator, or `self` if the sign move overflows.
            pub const fn with_pos_den_or_self(self) -> Self {
                match self.with_pos_den() { Ok(r) => r, Err(_) => self }
            }
            #[compile(same($is_signed, true))]
            /// Returns a reduced copy with a positive denominator.
            ///
            /// Can return `Overflow`.
            pub const fn reduced_pos_den(self) -> RatioResult<Self> {
                unwrap![ok? self.reduced()].with_pos_den()
            }
            #[compile(same($is_signed, true))]
            /// Returns a reduced copy with a positive denominator,
            /// or `self` if the transform overflows.
            pub const fn reduced_pos_den_or_self(self) -> Self {
                match self.reduced_pos_den() { Ok(r) => r, Err(_) => self }
            }

            /* conversion */

            /// Returns an `f32` approximation.
            pub const fn as_f32(self) -> f32 {
                let (n, d) = (self.num() as f32, self.den() as f32); n / d
            }
            /// Returns an `f64` approximation.
            pub const fn as_f64(self) -> f64 {
                let (n, d) = (self.num() as f64, self.den() as f64); n / d
            }

            /* ops */

            /// Composes two ratios by multiplication.
            ///
            /// This reduces cross terms to reduce the chance of overflowing.
            pub const fn compose(self, other: Self) -> RatioResult<Self> {
                if cif!(same($is_signed, true)) && (self._has_min_term() || other._has_min_term()) {
                    cold_path(); return Err(Overflow(None));
                }
                let g1 = Self::_gcd_pair_unchecked(self.n, other.den());
                let g2 = Self::_gcd_pair_unchecked(other.n, self.den());
                let (a, b) = (self.n / g1, self.den() / g2);
                let (c, d) = (other.n / g2, other.den() / g1);
                let n = unwrap![ok? Self::_mul_prim(a, c)];
                let d_raw = unwrap![ok? Self::_mul_prim(b, d)];
                let Some(d) = <$D>::new(d_raw) else {
                    cold_path();
                    return Err(Overflow(None));
                };
                Ok(Self { n, d })
            }
            /// Composes two ratios by multiplying their stored terms directly.
            ///
            /// This preserves the stored term structure.
            pub const fn compose_raw(self, other: Self) -> RatioResult<Self> {
                let n = unwrap![ok? Self::_mul_prim(self.n, other.n)];
                let d_raw = unwrap![ok? Self::_mul_prim(self.den(), other.den())];
                let Some(d) = <$D>::new(d_raw) else { cold_path(); return Err(Overflow(None)); };
                Ok(Self { n, d })
            }

            // apply
            /// Applies the ratio to `value`, rounding down.
            ///
            /// Can return `Overflow`.
            pub const fn apply_floor(self, value: $N) -> RatioResult<$N> {
                let n = unwrap![ok? Self::_mul_up(value, self.n)];
                let q = unwrap![ok? Self::_div_floor_up(n, self.d.get() as $Upcasted)];
                Self::up_to_prim(q)
            }
            /// Applies the ratio to `value`, rounding down to a multiple of `step`.
            pub const fn apply_floor_step(self, value: $N, step: $N) -> RatioResult<$N> {
                self._apply_step(value, step, 0)
            }

            /// Applies the ratio to `value`, rounding up.
            ///
            /// Can return `Overflow`.
            pub const fn apply_ceil(self, value: $N) -> RatioResult<$N> {
                let n = unwrap![ok? Self::_mul_up(value, self.n)];
                let q = unwrap![ok? Self::_div_ceil_up(n, self.d.get() as $Upcasted)];
                Self::up_to_prim(q)
            }
            /// Applies the ratio to `value`, rounding up to a multiple of `step`.
            pub const fn apply_ceil_step(self, value: $N, step: $N) -> RatioResult<$N> {
                self._apply_step(value, step, 1)
            }

            /// Applies the ratio to `value`, rounding to nearest, with ties away from zero.
            ///
            /// Can return `Overflow`.
            pub const fn apply_round(self, value: $N) -> RatioResult<$N> {
                let n = unwrap![ok? Self::_mul_up(value, self.n)];
                let q = unwrap![ok? Self::_div_round_away_up(n, self.d.get() as $Upcasted)];
                Self::up_to_prim(q)
            }
            /// Applies the ratio to `value`, rounding to the nearest multiple of `step`.
            pub const fn apply_round_step(self, value: $N, step: $N) -> RatioResult<$N> {
                self._apply_step(value, step, 2)
            }

            // inverse apply
            /// Applies the inverse ratio to `value`, rounding down.
            ///
            /// Can return `NoInverse` or `Overflow`.
            pub const fn inverse_apply_floor(self, value: $N) -> RatioResult<$N> {
                let r = unwrap![ok? self.recip()];
                r.apply_floor(value)
            }
            /// Applies the inverse ratio to `value`, rounding down to a multiple of `step`.
            pub const fn inverse_apply_floor_step(self, value: $N, step: $N) -> RatioResult<$N> {
                let r = unwrap![ok? self.recip()];
                r._apply_step(value, step, 0)
            }

            /// Applies the inverse ratio to `value`, rounding up.
            ///
            /// Can return `NoInverse` or `Overflow`.
            pub const fn inverse_apply_ceil(self, value: $N) -> RatioResult<$N> {
                let r = unwrap![ok? self.recip()];
                r.apply_ceil(value)
            }
            /// Applies the inverse ratio to `value`, rounding up to a multiple of `step`.
            pub const fn inverse_apply_ceil_step(self, value: $N, step: $N) -> RatioResult<$N> {
                let r = unwrap![ok? self.recip()];
                r._apply_step(value, step, 1)
            }

            /// Applies the inverse ratio to `value`, rounding to nearest, with ties away from zero.
            ///
            /// Can return `NoInverse` or `Overflow`.
            pub const fn inverse_apply_round(self, value: $N) -> RatioResult<$N> {
                let r = unwrap![ok? self.recip()];
                r.apply_round(value)
            }
            /// Applies the inverse ratio to `value`, rounding to the nearest multiple of `step`.
            pub const fn inverse_apply_round_step(self, value: $N, step: $N) -> RatioResult<$N> {
                let r = unwrap![ok? self.recip()];
                r._apply_step(value, step, 2)
            }

            /// Returns a ratio with the given denominator, rounding the numerator down.
            pub const fn with_den_floor(self, den: $N) -> RatioResult<Self> {
                let num = unwrap![ok? self.apply_floor(den)];
                Self::try_new(num, den)
            }
            /// Returns a ratio with the given denominator, rounding the numerator up.
            pub const fn with_den_ceil(self, den: $N) -> RatioResult<Self> {
                let num = unwrap![ok? self.apply_ceil(den)];
                Self::try_new(num, den)
            }
            /// Returns a ratio with the given denominator, rounding the numerator to nearest.
            pub const fn with_den_round(self, den: $N) -> RatioResult<Self> {
                let num = unwrap![ok? self.apply_round(den)];
                Self::try_new(num, den)
            }

            /// Returns a ratio with the given numerator, rounding the denominator down.
            pub const fn with_num_floor(self, num: $N) -> RatioResult<Self> {
                let den = unwrap![ok? self.inverse_apply_floor(num)];
                Self::try_new(num, den)
            }
            /// Returns a ratio with the given numerator, rounding the denominator up.
            pub const fn with_num_ceil(self, num: $N) -> RatioResult<Self> {
                let den = unwrap![ok? self.inverse_apply_ceil(num)];
                Self::try_new(num, den)
            }
            /// Returns a ratio with the given numerator, rounding the denominator to nearest.
            pub const fn with_num_round(self, num: $N) -> RatioResult<Self> {
                let den = unwrap![ok? self.inverse_apply_round(num)];
                Self::try_new(num, den)
            }

            // gcd, lcm
            #[compile(same($is_signed, false))]
            /// Returns the greatest common divisor of the stored terms.
            pub const fn gcd(self) -> $N { self._gcd_unchecked() }
            #[compile(same($is_signed, true))]
            /// Returns the greatest common divisor of the stored terms.
            ///
            /// Returns `Overflow` if a signed term is the minimum value.
            pub const fn gcd(self) -> RatioResult<$N> {
                if unlikely(self._has_min_term()) { return Err(Self::_err_overflow_pos()); }
                Ok(self._gcd_unchecked())
            }

            #[compile(same($is_signed, false))]
            /// Returns the least common multiple of the stored terms.
            ///
            /// Returns [`Overflow`] if the result is not representable by the primitive carrier.
            pub const fn lcm(self) -> RatioResult<$N> { self._lcm() }
            #[compile(same($is_signed, true))]
            /// Returns the least common multiple of the stored terms.
            ///
            /// Returns [`Overflow`] if a signed term is the minimum value,
            /// or if the result is not representable by the primitive carrier.
            pub const fn lcm(self) -> RatioResult<$N> {
                if unlikely(self._has_min_term()) { return Err(Overflow(None)); }
                self._lcm()
            }

            /* formatting */

            /// Writes the ratio into `buf` using `sep` between the stored terms.
            ///
            /// Returns the number of bytes written.
            ///
            /// Can return `MismatchedSizes` if `buf` is too small.
            pub const fn write_with_sep(self, buf: &mut [u8], sep: char) -> RatioResult<usize> {
                let mut offset = 0;
                offset = unwrap![ok? Self::_write_prim10(buf, offset, self.n)];
                offset = unwrap![ok? Self::_write_char(buf, offset, sep)];
                offset = unwrap![ok? Self::_write_prim10(buf, offset, self.den())];
                Ok(offset)
            }
            /// Writes the ratio into `buf` and returns it as `str`.
            ///
            /// Can return `MismatchedSizes` if `buf` is too small.
            ///
            /// # Features
            /// Uses `unsafe_str`, when enabled, to avoid duplicated UTF-8 validation.
            pub const fn write_str_with_sep<'a>(self, buf: &'a mut [u8], sep: char)
                -> RatioResult<&'a str> {
                let len = unwrap![ok? self.write_with_sep(buf, sep)];
                cfg_select! { all(feature = "unsafe_str", not(feature = "safe_num")) => {
                    // SAFETY: only decimal ASCII digits, `-`,
                    // and valid UTF-8 separator written by `Char::write_utf8_to`.
                    unsafe { Ok(Str::from_utf8_unchecked(slice![mut buf, ..len])) }
                } _ => {
                    Ok(unwrap![ok_guaranteed_or_ub Str::from_utf8(slice![mut buf, ..len])])
                }}
            }
        }

        // Private helpers
        impl $Name {
            const fn _den_one() -> $D { unwrap![some_guaranteed_or_ub <$D>::new(1)] }

            const fn _err_overflow() -> IntError { Overflow(None) }
            const fn _err_overflow_pos() -> IntError { Overflow(Some(Positive)) }
            const fn _err_overflow_neg() -> IntError { Overflow(Some(Negative)) }

            #[compile(same($is_signed, false))]
            const fn _is_positive(self) -> bool { self.n > 0 }
            #[compile(same($is_signed, true))]
            const fn _is_positive(self) -> bool {
                self.n != 0 && ((self.n > 0) == (self.d.get() > 0))
            }
            #[compile(same($is_signed, false))]
            const fn _is_negative(self) -> bool { false }
            #[compile(same($is_signed, true))]
            const fn _is_negative(self) -> bool {
                self.n != 0 && ((self.n > 0) != (self.d.get() > 0))
            }

            #[compile(same($is_signed, false))]
            const fn _num_den_abs(self) -> ($N, $N) { self.num_den() }
            #[compile(same($is_signed, true))]
            const fn _num_den_abs(self) -> ($N, $N) { (self.num().abs(), self.den().abs()) }

            #[compile(same($is_signed, false))]
            const fn _has_min_term(self) -> bool { false }
            #[compile(same($is_signed, true))]
            const fn _has_min_term(self) -> bool {
                self.num() == <$N>::MIN || self.den() == <$N>::MIN }

            // panics for signed ratios if a stored term is the minimum value.
            const fn _gcd_unchecked(self) -> $N {
                let (mut a, mut b) = self._num_den_abs();
                is![a == 0, return b];
                is![b == 0, return a];
                let k = (a | b).trailing_zeros();
                a >>= a.trailing_zeros();
                b >>= b.trailing_zeros();
                while b != 0 {
                    b >>= b.trailing_zeros();
                    is![a > b, cswap![mut: a, b], b -= a];
                }
                a << k
            }
            const fn _gcd_pair_unchecked(a: $N, b: $N) -> $N {
                let mut a = Self::_abs_prim_unchecked(a);
                let mut b = Self::_abs_prim_unchecked(b);
                is![a == 0, return b];
                is![b == 0, return a];
                let k = (a | b).trailing_zeros();
                a >>= a.trailing_zeros();
                b >>= b.trailing_zeros();
                while b != 0 {
                    b >>= b.trailing_zeros();
                    is![a > b, cswap![mut: a, b], b -= a];
                }
                a << k
            }

            const fn _lcm(self) -> RatioResult<$N> {
                let (a, b) = self._num_den_abs();
                if a == 0 { return Ok(0); }
                let g = self._gcd_unchecked();
                let aup = (a / g) as $Upcasted;
                let bup = b as $Upcasted;
                let Some(res) = aup.checked_mul(bup) else {
                    cold_path(); return Err(Overflow(None)); };
                is![res <= <$N>::MAX as $Upcasted, Ok(res as $N), Err(Overflow(None))]
            }

            #[compile(same($is_signed, false))]
            const fn _is_integer_checked(self) -> bool { self.n % self.d.get() == 0 }
            #[compile(same($is_signed, true))]
            const fn _is_integer_checked(self) -> bool {
                // Avoid signed overflow in MIN % -1.
                if self.n == <$N>::MIN && self.d.get() == -1 { cold_path(); return true; }
                self.n % self.d.get() == 0
            }

            /* modifications */

            const fn _reduced_unchecked(self) -> Self {
                let g = self._gcd_unchecked();
                let n = self.n / g;
                let d = self.d.get() / g;
                let d = unwrap![some_guaranteed_or_ub <$D>::new(d)]; // original den is nonzero
                Self { n, d }
            }

            #[compile(same($is_signed, false))]
            const fn _abs_prim_unchecked(x: $N) -> $N { x }
            #[compile(same($is_signed, true))]
            const fn _abs_prim_unchecked(x: $N) -> $N { x.abs() }
            #[compile(same($is_signed, true))]
            const fn abs_up(x: $Upcasted) -> RatioResult<$Upcasted> {
                if x == <$Upcasted>::MIN { cold_path(); Err(Overflow(None)) }
                else if x < 0 { Ok(-x) } else { Ok(x) }
            }

            #[compile(same($is_signed, false))]
            const fn up_to_prim(x: $Upcasted) -> RatioResult<$N> {
                is![x <= <$N>::MAX as $Upcasted, Ok(x as $N), Err(Self::_err_overflow_pos())]
            }
            #[compile(same($is_signed, true))]
            const fn up_to_prim(x: $Upcasted) -> RatioResult<$N> {
                is![x < <$N>::MIN as $Upcasted || x > <$N>::MAX as $Upcasted,
                    Err(Self::_err_overflow_pos()), Ok(x as $N)]
            }

            /* ops */

            const fn _apply_step(self, value: $N, step: $N, mode: u8) -> RatioResult<$N> {
                let step = unwrap![ok? Self::_checked_step(step)];
                // (value * n / d) / step = value * n / (d * step)
                let d_raw = unwrap![ok? Self::_mul_prim(self.den(), step)];
                let Some(d) = <$D>::new(d_raw) else { cold_path(); return Err(Overflow(None)); };
                let scaled = Self { n: self.n, d };
                let units = match mode {
                    0 => unwrap![ok? scaled.apply_floor(value)],
                    1 => unwrap![ok? scaled.apply_ceil(value)],
                    _ => unwrap![ok? scaled.apply_round(value)],
                };
                Self::_mul_prim(units, step)
            }

            #[compile(same($is_signed, false))]
            const fn _checked_step(step: $N) -> RatioResult<$N> {
                if step == 0 { cold_path(); Err(NonZeroRequired) } else { Ok(step) }
            }
            #[compile(same($is_signed, true))]
            const fn _checked_step(step: $N) -> RatioResult<$N> {
                if step == 0 { cold_path(); Err(NonZeroRequired) }
                else if step < 0 { cold_path(); Err(NonNegativeRequired) }
                else { Ok(step) }
            }

            const fn _mul_up(a: $N, b: $N) -> RatioResult<$Upcasted> {
                unwrap![some_ok_or (a as $Upcasted).checked_mul(b as $Upcasted), Overflow(None)]
            }
            const fn _mul_prim(a: $N, b: $N) -> RatioResult<$N> {
                let x = unwrap![ok? Self::_mul_up(a, b)];
                Self::up_to_prim(x)
            }

            #[compile(same($is_signed, false))]
            const fn _div_floor_up(a: $Upcasted, b: $Upcasted) -> RatioResult<$Upcasted> {
                Ok(a / b)
            }
            #[compile(same($is_signed, true))]
            const fn _div_floor_up(a: $Upcasted, b: $Upcasted) -> RatioResult<$Upcasted> {
                if a == <$Upcasted>::MIN && b == -1 {
                    cold_path(); return Err(Self::_err_overflow_pos()); }
                let (q, r) = (a / b, a % b);
                Ok(is![r != 0 && ((r > 0) != (b > 0)), q - 1, q])
            }

            #[compile(same($is_signed, false))]
            const fn _div_ceil_up(a: $Upcasted, b: $Upcasted) -> RatioResult<$Upcasted> {
                let (q, r) = (a / b, a % b);
                is![r != 0, Ok(q + 1), Ok(q)]
            }
            #[compile(same($is_signed, true))]
            const fn _div_ceil_up(a: $Upcasted, b: $Upcasted) -> RatioResult<$Upcasted> {
                // Avoid signed overflow in MIN % -1.
                if a == <$Upcasted>::MIN && b == -1 { cold_path(); return Err(Overflow(None)); }
                let (q, r) = (a / b, a % b);
                is![r != 0 && ((r > 0) == (b > 0)), Ok(q + 1), Ok(q)]
            }

            #[compile(same($is_signed, false))]
            const fn _div_round_away_up(a: $Upcasted, b: $Upcasted) -> RatioResult<$Upcasted> {
                let (q, r) = (a / b, a % b);
                let half = b / 2;
                let tie = b % 2 == 0 && r == half;
                let above = r > half;
                if above || tie { unwrap![some_ok_or q.checked_add(1), Overflow(None)] }
                else { Ok(q) }
            }
            #[compile(same($is_signed, true))]
            const fn _div_round_away_up(a: $Upcasted, b: $Upcasted) -> RatioResult<$Upcasted> {
                if a == <$Upcasted>::MIN && b == -1 {
                    cold_path(); return Err(Self::_err_overflow_pos()); }
                let (q, r) = (a / b, a % b);
                if r == 0 { return Ok(q); }
                let Ok(ar) = Self::abs_up(r) else { return Err(Overflow(None)); };
                let Ok(ab) = Self::abs_up(b) else { return Err(Overflow(None)); };
                let half = ab / 2;
                let tie = ab % 2 == 0 && ar == half;
                let above = ar > half;
                if above || tie {
                    unwrap![some_ok_or
                        if (a > 0) == (b > 0) { q.checked_add(1) } else { q.checked_sub(1) },
                        Overflow(None)]
                } else {
                    Ok(q)
                }
            }

            /* formatting */

            const fn _write_char(buf: &mut [u8], offset: usize, ch: char) -> RatioResult<usize> {
                let written = Char(ch).write_utf8_to(slice![mut buf, offset,..]);
                if written == 0 { cold_path(); Err(MismatchedSizes) }
                else { Ok(offset + written) }
            }
            #[compile(same($is_signed, false))]
            const fn _write_prim10(buf: &mut [u8], offset: usize, value: $N) -> RatioResult<usize> {
                let written = Digits(value).write_digits10(buf, offset);
                if written == 0 { cold_path(); Err(MismatchedSizes) }
                else { Ok(offset + written) }
            }
            #[compile(same($is_signed, true))]
            const fn _write_prim10(buf: &mut [u8], mut offset: usize, value: $N)
                -> RatioResult<usize> {
                let mag: $Unsigned;
                if value < 0 {
                    offset = unwrap![ok? Self::_write_char(buf, offset, '-')];
                    mag = value.unsigned_abs() as $Unsigned;
                } else { mag = value as $Unsigned; }
                let written = Digits(mag).write_digits10(buf, offset);
                if written == 0 { cold_path(); Err(MismatchedSizes) }
                else { Ok(offset + written) }
            }
        }
    };
}
_impl_ratio_prim!();
