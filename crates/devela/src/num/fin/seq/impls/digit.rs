//
//! Digit-based sequence method generation.
//

#[doc(hidden)]
#[macro_export]
macro_rules! __seq_impl_digit· {
    ($T:ident; thue_morse) => {
        /// Returns the `n`th term of the [Thue–Morse sequence],
        /// the parity of the binary digits of `n` (0 or 1).
        ///
        /// [Thue–Morse sequence]: https://mathworld.wolfram.com/Thue-MorseSequence.html
        ///
        /// See also [OEIS A010060](https://oeis.org/A010060).
        #[must_use]
        pub const fn thue_morse(n: $T) -> u8 {
            (n.count_ones() & 1) as u8
        }
    };
    ($T:ident; prouhet) => {
        /// Returns the `n`th term of the generalized Prouhet–Thue–Morse sequence.
        ///
        /// Computes the base-`radix` digit sum of `n`, modulo `radix`.
        /// Returns `None` for radices below 2.
        ///
        /// See the [Prouhet–Tarry–Escott problem] and
        /// [OEIS A141803](https://oeis.org/A141803).
        ///
        /// [Prouhet–Tarry–Escott problem]: https://en.wikipedia.org/wiki/Prouhet%E2%80%93Tarry%E2%80%93Escott_problem
        #[must_use]
        pub const fn prouhet(mut n: $T, radix: $T) -> Option<$T> {
            $crate::is! { radix < 2, return None }
            // The digit sum cannot exceed the original index.
            let mut sum: $T = 0;
            while n != 0 {
                sum += n % radix;
                n /= radix;
            }
            Some(sum % radix)
        }
    };
    ($T:ident; gray) => {
        /// Returns the binary-reflected [Gray code] at index `n`.
        ///
        /// Consecutive terms differ in exactly one bit.
        ///
        /// [Gray code]: https://en.wikipedia.org/wiki/Gray_code
        ///
        /// See also [OEIS A003188](https://oeis.org/A003188).
        #[must_use]
        pub const fn gray(n: $T) -> $T {
            n ^ (n >> 1)
        }
    };
    ($T:ident; van_der_corput) => {
        /// Returns the `n`th term of the [Van der Corput sequence] in base `radix`.
        ///
        /// The radical inverse is represented as an exact, unreduced ratio
        /// in `[0, 1)`, with a nonzero denominator.
        ///
        /// Returns `None` for radices below 2 or if the unreduced
        /// denominator overflows the carrier.
        ///
        /// [Van der Corput sequence]: https://en.wikipedia.org/wiki/Van_der_Corput_sequence
        #[must_use]
        pub const fn van_der_corput(
            mut n: $T,
            radix: $T,
        ) -> Option<$crate::Ratio<$T, $crate::NonZero<$T>>> {
            $crate::is! { radix < 2, return None }
            let (mut num, mut den): ($T, $T) = (0, 1);
            while n != 0 {
                den = $crate::unwrap![some? den.checked_mul(radix)];
                // The numerator remains strictly below the checked denominator.
                num = num * radix + n % radix;
                n /= radix;
            }
            let Some(den) = $crate::NonZero::<$T>::new(den) else {
                return None;
            };
            Some($crate::Ratio::from_parts(num, den))
        }
    };
}
pub use __seq_impl_digit· as __seq_impl_digit;
