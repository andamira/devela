// devela::num::int::wrapper::impl_prime
//
//! implements prime-related integer methods
//
// TOC
// - signed|unsigned:
//   - is_prime
//   - prime_nth
//   - prime_pi
//   - totient

use crate::{
    code::{iif, paste},
    num::{isize_up, usize_up, Int, NumErrors, NumResult as Result},
};
use NumErrors::Overflow;

// $t:   the input/output type
// $up:  the upcasted type to do the operations on (for prime_pi)
// $d:  the doclink suffix for the method name
macro_rules! impl_int {
    (signed $( $t:ty : $up:ty : $d:literal ),+) => { $( impl_int![@signed $t:$up:$d]; )+ };
    (unsigned $( $t:ty : $up:ty : $d:literal ),+) => { $( impl_int![@unsigned $t:$up:$d]; )+ };

    // implements signed ops
    (@signed $t:ty : $up: ty : $d:literal) => { paste! {
        #[doc = "# Integer prime-related methods for `" $t "`\n\n"]
        #[doc = "- [is_prime](#method.is_prime" $d ")"]
        #[doc = "- [prime_nth](#method.prime_nth" $d ")"]
        #[doc = "- [prime_pi](#method.prime_pi" $d ")"]
        #[doc = "- [totient](#method.totient" $d ")"]
        ///
        /// See the related trait [`NumInt`][crate::num::NumInt].
        impl Int<$t> {
            /// Returns `true` if `n` is prime.
            ///
            /// This approach uses optimized trial division, which means it checks
            /// only odd numbers starting from 3 and up to the square root of the
            /// given number. This is based on the fact that if a number is
            /// divisible by a number larger than its square root, the result of the
            /// division will be smaller than the square root, and it would have
            /// already been checked in previous iterations.
            /// # Examples
            /// ```
            /// # use devela::num::Int;
            #[doc = "assert![Int(127_" $t ").is_prime()];"]
            #[doc = "assert![Int(2_" $t ").is_prime()];"]
            #[doc = "assert![!Int(1_" $t ").is_prime()];"]
            #[doc = "assert![!Int(-2_" $t ").is_prime()];"]
            /// ```
            #[must_use] #[inline]
            pub const fn is_prime(self) -> bool {
                match self.0 {
                    ..=1 =>  false,
                    2..=3 => true,
                    _ => {
                        iif![self.0 % 2 == 0; return false];
                        let limit = iif![let Ok(s) = self.sqrt_floor(); s.0; unreachable!()];
                        let mut i = 3;
                        while i <= limit { iif![self.0 % i == 0; return false]; i += 2; }
                        true
                    }
                }
            }

            /// Finds the 0-indexed `nth` prime number.
            ///
            /// Note: If `nth` is negative, this function treats it as its absolute
            /// value. For example, a value of `-3` will be treated as `3`, and the
            /// function will return the 3rd prime number.
            /// # Errors
            /// Returns [`Overflow`] if the result can't fit the type.
            /// # Examples
            /// ```
            /// # use devela::num::Int;
            #[doc = "assert_eq![Ok(Int(2)), Int(0_" $t ").prime_nth()];"]
            #[doc = "assert_eq![Ok(Int(3)), Int(1_" $t ").prime_nth()];"]
            #[doc = "assert_eq![Ok(Int(127)), Int(30_" $t ").prime_nth()];"]
            #[doc = "assert_eq![Ok(Int(127)), Int(-30_" $t ").prime_nth()];"]
            /// assert![Int(31_i8).prime_nth().is_err()];
            /// ```
            #[inline]
            pub const fn prime_nth(self) -> Result<Int<$t>> {
                let [nth, mut count, mut i] = [self.0.abs(), 1, 2];
                loop {
                    if Int(i).is_prime() {
                        iif![count - 1 == nth; return Ok(Int(i))];
                        count += 1;
                    }
                    i = iif![let Some(i) = i.checked_add(1); i; return Err(Overflow(None))];
                }
            }

            /// Counts the number of primes upto and including `n`.
            ///
            #[doc = "It upcasts internally to [`" $up "`] for the inner operations."]
            /// # Panics
            /// It can panic if `n == i128|u128`, at the last iteration of a loop
            /// that would take an unfeasable amount of time.
            /// # Notation
            /// $$\pi(x)$$
            /// # Examples
            /// ```
            /// # use devela::num::Int;
            #[doc = "assert_eq![1, Int(2_" $t ").prime_pi()];"]
            #[doc = "assert_eq![2, Int(3_" $t ").prime_pi()];"]
            #[doc = "assert_eq![31, Int(127_" $t ").prime_pi()];"]
            #[doc = "assert_eq![0, Int(-5_" $t ").prime_pi()];"]
            /// ```
            /// # Links
            /// - <https://mathworld.wolfram.com/PrimeCountingFunction.html>.
            /// - <https://en.wikipedia.org/wiki/Prime-counting_function>.
            #[inline]
            pub const fn prime_pi(self) -> usize {
                let (mut prime_count, mut i) = (0_usize, 0 as $up);
                while i <= self.0 as $up {
                    iif![Int(i as $t).is_prime(); prime_count += 1];
                    i += 1;
                }
                prime_count
            }

            /// Counts the number of integers $<|n|$ that are relatively prime to `n`.
            ///
            /// Note: If `n` is negative, this function treats it as its absolute
            /// value. For example, a value of `-3` will be treated as `3`.
            /// # Algorithm
            /// This function iterates through all numbers from 2 up to the square
            /// root of $|n|$. If it finds a divisor, it reduces `n` by its factors
            /// and adjusts result accordingly. If after the loop, $n > 1$, it
            /// means `n` has a prime factor greater than its square root, and the
            /// function adjusts result for this last factor.
            /// $$\large\varphi(n) =n \prod_{p\mid |n|} \left(1-\frac{1}{p}\right)$$
            /// # Examples
            /// ```
            /// # use devela::num::Int;
            #[doc = "assert_eq![Int(2), Int(4_" $t ").totient()];"]
            #[doc = "assert_eq![Int(6), Int(9_" $t ").totient()];"]
            #[doc = "assert_eq![Int(12), Int(13_" $t ").totient()];"]
            #[doc = "assert_eq![Int(22), Int(-23_" $t ").totient()];"]
            #[doc = "assert_eq![Int(2), Int(-3_" $t ").totient()];"]
            /// ```
            /// # Links
            /// - <https://en.wikipedia.org/wiki/Euler%27s_totient_function>.
            #[must_use] #[inline]
            pub const fn totient(self) -> Int<$t> {
                let (mut n, mut result, mut i) = (self.0.abs(), self.0.abs(), 2);
                while i * i <= n {
                    if n % i == 0 {
                        while n % i == 0 { n /= i; }
                        result -= result / i;
                    }
                    i += 1;
                }
                iif![n > 1; result -= result / n];
                Int(result)
            }
        }
    }};

    // implements unsigned ops
    (@unsigned $t:ty : $up:ty : $d:literal) => { paste! {
        #[doc = "# Integer prime-related methods for `" $t "`\n\n"]
        #[doc = "- [is_prime](#method.is_prime" $d ")"]
        #[doc = "- [prime_nth](#method.prime_nth" $d ")"]
        #[doc = "- [prime_pi](#method.prime_pi" $d ")"]
        #[doc = "- [totient](#method.totient" $d ")"]
        ///
        /// See the related trait [`NumInt`][crate::num::NumInt].
        impl Int<$t> {
            /// Returns `true` if `n` is prime.
            ///
            /// This approach uses optimized trial division, which means it checks
            /// only odd numbers starting from 3 and up to the square root of the
            /// given number. This is based on the fact that if a number is
            /// divisible by a number larger than its square root, the result of the
            /// division will be smaller than the square root, and it would have
            /// already been checked in previous iterations.
            /// # Examples
            /// ```
            /// # use devela::num::Int;
            #[doc = "assert![Int(127_" $t ").is_prime()];"]
            #[doc = "assert![Int(2_" $t ").is_prime()];"]
            #[doc = "assert![!Int(1_" $t ").is_prime()];"]
            /// ```
            #[must_use] #[inline]
            pub const fn is_prime(self) -> bool {
                match self.0 {
                    ..=1 =>  false,
                    2..=3 => true,
                    _ => {
                        iif![self.0 % 2 == 0; return false];
                        let limit = self.sqrt_floor().0;
                        let mut i = 3;
                        while i <= limit { iif![self.0 % i == 0; return false]; i += 2; }
                        true
                    }
                }
            }

            /// Finds the 0-indexed `nth` prime number.
            /// # Errors
            /// Returns [`Overflow`] if the result can't fit the type.
            /// # Examples
            /// ```
            /// # use devela::num::Int;
            #[doc = "assert_eq![Ok(Int(2)), Int(0_" $t ").prime_nth()];"]
            #[doc = "assert_eq![Ok(Int(3)), Int(1_" $t ").prime_nth()];"]
            #[doc = "assert_eq![Ok(Int(251)), Int(53_" $t ").prime_nth()];"]
            /// assert![Int(54_u8).prime_nth().is_err()];
            /// ```
            #[inline]
            pub const fn prime_nth(self) -> Result<Int<$t>> {
                let [nth, mut count, mut i] = [self.0, 1, 2];
                loop {
                    if Int(i).is_prime() {
                        iif![count - 1 == nth; return Ok(Int(i))];
                        count += 1;
                    }
                    i = iif![let Some(i) = i.checked_add(1); i; return Err(Overflow(None))];
                }
            }

            /// Counts the number of primes upto and including `n`.
            ///
            #[doc = "It upcasts internally to [`" $up "`] for the inner operations."]
            /// # Panics
            /// It can panic if `n == i128|u128`, at the last iteration of a loop
            /// that would take an unfeasable amount of time.
            /// # Notation
            /// $$\pi(x)$$
            /// # Examples
            /// ```
            /// # use devela::num::Int;
            #[doc = "assert_eq![1, Int(2_" $t ").prime_pi()];"]
            #[doc = "assert_eq![2, Int(3_" $t ").prime_pi()];"]
            #[doc = "assert_eq![31, Int(127_" $t ").prime_pi()];"]
            /// ```
            /// # Links
            /// - <https://mathworld.wolfram.com/PrimeCountingFunction.html>.
            /// - <https://en.wikipedia.org/wiki/Prime-counting_function>.
            #[inline]
            pub const fn prime_pi(self) -> usize {
                let (mut prime_count, mut i) = (0_usize, 0 as $up);
                while i <= self.0 as $up {
                    iif![Int(i as $t).is_prime(); prime_count += 1];
                    i += 1;
                }
                prime_count
            }

            /// Counts the number of integers $<n$ that are relatively prime to `n`.
            /// # Algorithm
            /// This function iterates through all numbers from 2 up to the square
            /// root of $n$. If it finds a divisor, it reduces `n` by its factors
            /// and adjusts result accordingly. If after the loop, $n > 1$, it
            /// means `n` has a prime factor greater than its square root, and the
            /// function adjusts result for this last factor.
            /// $$\large\varphi(n) =n \prod_{p\mid n} \left(1-\frac{1}{p}\right)$$
            /// # Examples
            /// ```
            /// # use devela::num::Int;
            #[doc = "assert_eq![Int(2), Int(4_" $t ").totient()];"]
            #[doc = "assert_eq![Int(6), Int(9_" $t ").totient()];"]
            #[doc = "assert_eq![Int(12), Int(13_" $t ").totient()];"]
            /// ```
            /// # Links
            /// - <https://en.wikipedia.org/wiki/Euler%27s_totient_function>.
            #[must_use] #[inline]
            pub const fn totient(self) -> Int<$t> {
                let (mut n, mut result, mut i) = (self.0, self.0, 2);
                while i * i <= n {
                    if n % i == 0 {
                        while n % i == 0 { n /= i; }
                        result -= result / i;
                    }
                    i += 1;
                }
                iif![n > 1; result -= result / n];
                Int(result)
            }
        }
    }};

    // $n:  the niche type name prefix (e.g. NonRange)
    // $t:  the niche inner type (the associated primitive integer) (e.g. u8)
    // $($g)*: an optional list of const generics (e.g. RMIN, RMAX)
    // $d:  the doclink suffix for the method name
    // $dt: the doclink suffix for the associated method name implemented for the inner primitive
    (niche $( $n:ident : $t:ident <$($g:ident),*> : $d:literal : $dt: literal),+ $(,)? ) => {
        $( impl_int![@niche $n:$t <$($g),*> : $d:$dt ]; )+
    };
    (@niche $n:ident : $t:ident <$($g:ident),*> : $d:literal : $dt: literal) => { paste! {
        #[doc = "# Integer prime-related methods for `" $t "`\n\n"]
        #[doc = "- [is_prime](#method.is_prime" $d ")"]
        #[doc = "- [prime_nth](#method.prime_nth" $d ")"]
        #[doc = "- [prime_pi](#method.prime_pi" $d ")"]
        #[doc = "- [totient](#method.totient" $d ")"]
        ///
        /// See the related trait [`NumInt`][crate::num::NumInt].
        impl<$(const $g:$t,)*> Int<[<$n$t:camel>]<$($g,)*>> {
            num_niche_impls![Int=>bool: $n:$t:$dt<$($g),*>, +const is_prime, self];
            num_niche_impls![Int=>res $n:$t:$dt<$($g),*>, +const prime_nth, self];
            num_niche_impls![Int=>usize: $n:$t:$dt<$($g),*>, +const prime_pi, self];
            num_niche_impls![Int $n:$t:$dt<$($g),*>, +const totient, self];
        }
    }};
}
impl_int![signed
    i8:i16:"", i16:i32:"-1", i32:i64:"-2", i64:i128:"-3", i128:i128:"-4", isize:isize_up:"-5"];
impl_int![unsigned
    u8:u16:"-6", u16:u32:"-7", u32:u64:"-8", u64:u128:"-9", u128:u128:"-10", usize:usize_up:"-11"];

#[cfg(feature = "num_niche_impls")]
use crate::num::{niche::*, num_niche_impls};
#[cfg(feature = "num_niche_impls")]
num_niche_impls![impl_int niche];
