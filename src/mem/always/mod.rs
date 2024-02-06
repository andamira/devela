// devela::mem::always
//
//! Always available functionality, for internal use.
//

mod fns;

#[allow(unused)]
pub use fns::*;

/// Swaps two mutable references in compile-time, using a temporary variable.
// WAIT: [const_swap](https://github.com/rust-lang/rust/issues/83163)
#[macro_export]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "mem")))]
macro_rules! cswap {
    ($a:expr, $b:expr) => {{
        let tmp = $a;
        $a = $b;
        $b = tmp;
    }};
}
pub use cswap;
