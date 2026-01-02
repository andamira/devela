// devela_base_core::code::util::asserts::static::const
//
//! Compile-time assertions.
//

#[doc = crate::_TAG_ASSERT!()]
/// Asserts various comparisons on constants.
#[doc = crate::_doc_location!("code/util")]
#[macro_export]
#[cfg_attr(cargo_primary_package, doc(hidden))]
macro_rules! const_assert {
    (   /* logical comparisons */

        // Asserts that a single expression evaluates to `true`.
        $x:expr $(,)?) => {
        const _: $crate::True = $crate::const_bool!($x);
    };
    (
        // Asserts that constants are equal in value.
        eq $x:expr, $($y:expr),+ $(,)?) => {
        $crate::const_assert!($($x == $y)&&+);
    };
    (
        // Asserts that constants of type `usize` are equal in value.
        //
        // Allows inspecting the values of the constants in error messages. E.g.:
        // "expected an array with a size of 3, found one with a size of 4"
        eq_usize $x:expr, $($y:expr),+ $(,)?) => {
        $(const _: [(); $x] = [(); $y];)+
    };
    (
        // Asserts that constants are NOT equal in value.
        ne $x:expr, $($y:expr),+ $(,)?) => {
        $crate::const_assert!($($x != $y)&&+);
    };
    (
        // Asserts that constants are less than each other.
        lt $x:expr, $($y:expr),+ $(,)?) => {
        $crate::const_assert!(@build $x, $($y),+; <);
    };
    (
        // Asserts that constants are less than or equal to each other.
        le $x:expr, $($y:expr),+ $(,)?) => {
        $crate::const_assert!(@build $x, $($y),+; <=);
    };
    (
        // Asserts that constants are greater than each other.
        gt $x:expr, $($y:expr),+ $(,)?) => {
        $crate::const_assert!(@build $x, $($y),+; >);
    };
    (
        // Asserts that constants are greater than or equal to each other.
        ge $x:expr, $($y:expr),+ $(,)?) => {
        $crate::const_assert!(@build $x, $($y),+; >=);
    };
    (
        /* comparisons of slices of primitives (as well as slices of slices)
         * supports all integer primitives, bool, char and &str. */

        // Asserts that slice buffers of $p primitives are equal in value.
        eq_buf::<$p:ty> $buf:expr, $($other:expr),+ $(,)?) => {
        $( $crate::const_assert!($crate::Slice::<$p>::eq($buf, $other)); )+
    };
    (   // (defaults to bytes)
        eq_buf $buf:expr, $($other:expr),+ $(,)?) => {
        $( $crate::const_assert!($crate::Slice::<u8>::eq($buf, $other)); )+
    };
    (
        // Asserts that slice buffers of $p primitives are NOT equal in value.
        ne_buf::<$p:ty> $buf:expr, $($other:expr),+ $(,)?) => {
        $( $crate::const_assert!(! $crate::Slice::<$p>::eq($buf, $other)); )+
    };
    (   // (defaults to bytes)
        ne_buf $buf:expr, $($other:expr),+ $(,)?) => {
        $( $crate::const_assert!(! $crate::Slice::<u8>::eq($buf, $other)); )+
    };
    (
        /* comparisons of string slices (for convenience) */

        // Asserts that string slices are equal in value.
        eq_str $str:expr, $($other:expr),+ $(,)?) => {
        $( $crate::const_assert!($crate::Slice::<&str>::eq($str, $other)); )+
    };
    (
        // Asserts that string slices are NOT equal in value.
        ne_str $str:expr, $($other:expr),+ $(,)?) => {
        $( $crate::const_assert!(! $crate::Slice::<&str>::eq($str, $other)); )+
    };

    // (receives the expressions and the operator, and performs the appropriate comparison)
    (   /* private arms*/
     @build $x:expr, $($y:expr),+; $op:tt) => {
        $crate::const_assert!($x $op $crate::capture_first!(expr $($y),+));
        $crate::const_assert!(@build $($y),+; $op);
    };
    // (terminates recursion when there’s only one expression left)
    (@build $x:expr; $op:tt) => {};
}
#[doc(inline)]
pub use const_assert;

#[cfg(test)]
mod tests {
    #[test]
    const fn const_assert_logic() {
        const_assert!(true && (true != false));
        const_assert!((true && true) != false);

        const_assert!(eq false, false);

        const SIZE_A: usize = 3;
        const SIZE_B: usize = 3;
        const_assert!(eq_usize SIZE_A, SIZE_B);

        const_assert!(ne true, false);

        #[allow(dead_code)]
        const FIVE: usize = 5;

        const_assert!(FIVE * 2 == 10);
        const_assert!(FIVE > 2);

        const_assert!(le 1, 1, 2, 3, 4, 4);
        const_assert!(lt 1, 2, 3);
        const_assert!(ge 4, 4, 3, 2, 1, 1);
        const_assert!(gt 4, 3, 2, 1);
    }
    #[test]
    #[rustfmt::skip] // reason: rustfmt gets confused with the first `&` and splits it
    const fn const_assert_slices() {
        /* primitive slices */

        const_assert!(eq_buf::<i32> &[-1, 2], &[-1, 2]);
        const_assert!(ne_buf::<i32> &[-1, 2], &[-3, 4]);

        const_assert!(eq_buf::<bool> &[true, false, true], &[true, false, true]);
        const_assert!(ne_buf::<bool> &[true, false, true], &[true, true, false]);

        const_assert!(eq_buf::<char> &['a', 'b', 'c'], &['a', 'b', 'c']);
        const_assert!(ne_buf::<char> &['a', 'b', 'c'], &['b', 'c', 'a']);

        /* slices of slices of primitives */

        const_assert!(eq_buf::<&[i32]> &[&[-1, 2], &[3, 4]], &[&[-1, 2], &[3, 4]]);
        const_assert!(ne_buf::<&[i32]> &[&[-1, 2], &[3, 4]], &[&[-3, 4], &[1, 2]]);

        const_assert!(eq_buf::<&[bool]> &[&[true], &[false]], &[&[true], &[false]]);
        const_assert!(ne_buf::<&[bool]> &[&[true], &[false]], &[&[false], &[true]]);

        const_assert!(eq_buf::<&[char]> &[&['a', 'b'], &['c', 'd']], &[&['a', 'b'], &['c', 'd']]);
        const_assert!(ne_buf::<&[char]> &[&['a', 'b'], &['c', 'd']], &[&['a', 'b'], &['d', 'c']]);

        /* byte slices (default case) */

        const_assert!(eq_buf b"hello", b"hello");
        const_assert!(ne_buf b"hello", b"world");

        const_assert!(eq_buf &[1, 2], &[1, 2]);
        const_assert!(ne_buf &[1, 2], &[3, 4]);

        /* slices of byte slices */

        const_assert!(eq_buf::<&[u8]> &[&[1, 2], &[3, 4]], &[&[1, 2], &[3, 4]]);
        const_assert!(ne_buf::<&[u8]> &[&[1, 2], &[3, 4]], &[&[3, 4], &[1, 2]]);

        const_assert!(eq_buf::<&[u8]> &[b"hi", b"ho"], &[b"hi", b"ho"]);
        const_assert!(ne_buf::<&[u8]> &[b"hi", b"ho"], &[b"ho", b"hi"]);

        /* string slices */

        const_assert!(eq_buf::<&str> "hello", "hello");
        const_assert!(ne_buf::<&str> "hello", "world");

        // convenience arm:
        const_assert!(eq_str "hello", "hello");
        const_assert!(ne_str "hello", "world");

        /* slices of string slices */

        const_assert!(eq_buf::<&[&str]> &["hello", "world"], &["hello", "world"]);
        const_assert!(ne_buf::<&[&str]> &["hello", "world"], &["world", "hello"]);
    }
}
