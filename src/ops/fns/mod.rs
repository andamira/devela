// devela::ops::fns
//
//! Functions for numeric operations.
//

mod base;
mod div_sqrt;
mod factors;
mod gcd_lcm;
mod scale_lerp;

pub use {base::*, div_sqrt::*, factors::*, gcd_lcm::*, scale_lerp::*};

/// Swaps two mutable references in *const* time, using a temporary variable.
#[macro_export]
macro_rules! cswap { ($a:expr, $b:expr) => {{ let tmp = $a; $a = $b; $b = tmp; }}; }
pub use cswap;
