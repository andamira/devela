// devela::mem::always
//
//! Always available functionality, for internal use.
//

mod fns;

#[allow(unused)]
pub use fns::*;

/// Swaps two mutable references in compilation time, using a temporary variable.
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
