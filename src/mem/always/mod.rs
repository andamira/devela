// devela::mem::always
//
//! Always available functionality, for internal use.
//

mod fns;

pub use fns::*;

/// Swaps two mutable references in *const* time, using a temporary variable.
#[macro_export]
macro_rules! cswap {
    ($a:expr, $b:expr) => {{
        let tmp = $a;
        $a = $b;
        $b = tmp;
    }};
}
pub use cswap;
