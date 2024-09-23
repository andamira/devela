// devela::code::asserts::static::const
//
//! Compile-time assertions.
//

/// Asserts various comparisons on constants.
#[macro_export]
#[cfg_attr(cargo_primary_package, doc(hidden))]
macro_rules! const_assert {
    (
        // Asserts that a single expression evaluates to `true`.
        $x:expr $(,)?) => {
        const _: $crate::True = $crate::num::const_bool!($x);
    };
    (
        // Asserts that constants are equal in value.
        eq $x:expr, $($y:expr),+ $(,)?) => {
        $crate::code::const_assert!($($x == $y)&&+);
    };
    (
        // Asserts that constants of type `usize` are equal in value.
        // (Allows for inspecting the values in error messages).
        eq_usize $x:expr, $($y:expr),+ $(,)?) => {
        $(const _: [(); $x] = [(); $y];)+
    };
    (
        // Asserts that constants are not equal in value.
        ne $x:expr, $($y:expr),+ $(,)?) => {
        $crate::code::const_assert!($($x != $y)&&+);
    };
   (
        // Asserts that constants are less than each other.
        lt $x:expr, $($y:expr),+ $(,)?) => {
        $crate::code::const_assert!(@build $x, $($y),+; <);
    };
    (
        // Asserts that constants are less than or equal to each other.
        le $x:expr, $($y:expr),+ $(,)?) => {
        $crate::code::const_assert!(@build $x, $($y),+; <=);
    };
    (
        // Asserts that constants are greater than each other.
        gt $x:expr, $($y:expr),+ $(,)?) => {
        $crate::code::const_assert!(@build $x, $($y),+; >);
    };
    (
        // Asserts that constants are greater than or equal to each other.
        ge $x:expr, $($y:expr),+ $(,)?) => {
        $crate::code::const_assert!(@build $x, $($y),+; >=);
    };

    /* private arms */

    // receives the expressions and the operator, and performs the appropriate comparison.
    (
        @build $x:expr, $($y:expr),+; $op:tt) => {
        $crate::code::const_assert!($x $op $crate::code::capture_first!(expr $($y),+));
        $crate::code::const_assert!(@build $($y),+; $op);
    };
    // terminates recursion when there’s only one expression left.
    (
        @build $x:expr; $op:tt) => {};
}
#[doc(inline)]
pub use const_assert;

#[cfg(test)]
mod tests {
    pub use super::*;

    #[test]
    const fn const_assert() {
        const_assert!(true && (true != false));
        const_assert!((true && true) != false);
        const_assert!(eq false, false);
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
}
