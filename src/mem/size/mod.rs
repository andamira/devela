// devela::mem::size
//
//! Functionality related to memory size.
//

/* contains always compiled items */

mod size;

#[allow(unused)]
#[cfg(not(feature = "mem"))]
pub(crate) use size::*;

/* feature-gated */

#[cfg(feature = "mem")]
mod bit;
#[cfg(feature = "mem")]
mod expr;

// re-export private sub-modules
#[cfg(feature = "mem")]
pub use {bit::*, expr::*, size::*};
