// devela::mem::always
//
//! Always available functionality, for internal use.
//

mod fns;

#[allow(unused)]
pub use fns::*;

/// Swaps two mutable variables in a *compile-time* friendly manner.
// WAIT: [const_swap](https://github.com/rust-lang/rust/issues/83163)
#[macro_export]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "mem")))]
macro_rules! cswap {
    (
        // swaps any two mutable values by using a temporary variable
        $a:expr, $b:expr) => {{
        let tmp = $a;
        $a = $b;
        $b = tmp;
    }};
    (
        // swaps two `T: BitXorAssign` mutable values without needing a temporary variable
        xor $a:expr, $b:expr) => {{
        $a ^= $b;
        $b ^= $a;
        $a ^= $b;
    }};
}
pub use cswap;
