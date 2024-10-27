// devela::code::macros::asserts::static::const
//
//! Compile-time assertions.
//

/// Asserts various comparisons on constants.
#[macro_export]
#[cfg_attr(cargo_primary_package, doc(hidden))]
macro_rules! assert_const {
    (
        // Asserts that a single expression evaluates to `true`.
        $x:expr $(,)?) => {
        const _: $crate::True = $crate::num::const_bool!($x);
    };
    (
        // Asserts that constants are equal in value.
        eq $x:expr, $($y:expr),+ $(,)?) => {
        $crate::code::assert_const!($($x == $y)&&+);
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
        $crate::code::assert_const!($($x != $y)&&+);
    };
   (
        // Asserts that constants are less than each other.
        lt $x:expr, $($y:expr),+ $(,)?) => {
        $crate::code::assert_const!(@build $x, $($y),+; <);
    };
    (
        // Asserts that constants are less than or equal to each other.
        le $x:expr, $($y:expr),+ $(,)?) => {
        $crate::code::assert_const!(@build $x, $($y),+; <=);
    };
    (
        // Asserts that constants are greater than each other.
        gt $x:expr, $($y:expr),+ $(,)?) => {
        $crate::code::assert_const!(@build $x, $($y),+; >);
    };
    (
        // Asserts that constants are greater than or equal to each other.
        ge $x:expr, $($y:expr),+ $(,)?) => {
        $crate::code::assert_const!(@build $x, $($y),+; >=);
    };

    // receives the expressions and the operator, and performs the appropriate comparison.
    (
    /* private arms*/

        @build $x:expr, $($y:expr),+; $op:tt) => {
        $crate::code::assert_const!($x $op $crate::code::capture_first!(expr $($y),+));
        $crate::code::assert_const!(@build $($y),+; $op);
    };
    // terminates recursion when there’s only one expression left.
    (
        @build $x:expr; $op:tt) => {};
}
#[doc(inline)]
pub use assert_const;

#[cfg(test)]
mod tests {
    pub use super::*;

    #[test]
    const fn assert_const() {
        assert_const!(true && (true != false));
        assert_const!((true && true) != false);
        assert_const!(eq false, false);
        assert_const!(ne true, false);

        #[allow(dead_code)]
        const FIVE: usize = 5;

        assert_const!(FIVE * 2 == 10);
        assert_const!(FIVE > 2);

        assert_const!(le 1, 1, 2, 3, 4, 4);
        assert_const!(lt 1, 2, 3);
        assert_const!(ge 4, 4, 3, 2, 1, 1);
        assert_const!(gt 4, 3, 2, 1);
    }
}
