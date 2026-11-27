//
//! Recurrence-based sequence method generation.

#[doc(hidden)]
#[macro_export]
macro_rules! __seq_impl_recurrence· {
    ($T:ident; fibonacci) => {
        /// Returns the `n`th [Fibonacci number], with F(0) = 0 and F(1) = 1.
        ///
        /// Returns `None` if the result overflows the carrier.
        ///
        /// [Fibonacci number]: https://en.wikipedia.org/wiki/Fibonacci_sequence
        ///
        /// See also [OEIS A000045](https://oeis.org/A000045).
        #[must_use]
        pub const fn fibonacci(n: $T) -> Option<$T> {
            $crate::is! { n == 0, return Some(0) }
            let (mut a, mut b): ($T, $T) = (0, 1);
            $crate::whilst! { i in 1 as $T,..n; {
                let next = $crate::unwrap![some? a.checked_add(b)];
                a = b;
                b = next;
            }}
            Some(b)
        }
    };
}
pub use __seq_impl_recurrence· as __seq_impl_recurrence;
