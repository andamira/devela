// devela::mem::always
//
//! Always available functionality, for internal use.
//

mod fns;

#[allow(unused)]
pub use fns::*;

/// Swaps two mutable variables in a *compile-time* friendly manner.
///
/// For that it uses either a temporary variable or the [xor swap method].
///
/// [xor swap method]: https://en.wikipedia.org/wiki/XOR_swap_algorithm
// WAIT: [const_swap](https://github.com/rust-lang/rust/issues/83163)
#[macro_export]
macro_rules! cswap {
    (
        // swaps two values using a temporary variable
        $a:expr, $b:expr) => {{
        let tmp = $a;
        $a = $b;
        $b = tmp;
    }};
    (
        // swaps two `T: PartialEq + BitXorAssign` values without a temporary variable
        xor $a:expr, $b:expr) => {{
        if $a != $b {
            $a ^= $b;
            $b ^= $a;
            $a ^= $b;
        }
    }};
}
pub use cswap;
