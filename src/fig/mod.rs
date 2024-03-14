// devela::fig
//
//! Geometric figures and operations, spatial constructs and analysis.
//

// safety:
#![cfg_attr(feature = "safe_fig", forbid(unsafe_code))]

/* feature-gated */

// #[cfg(feature = "fig")]
// mod algebra;
#[cfg(feature = "fig")]
mod prim;

#[cfg(feature = "fig")]
pub use prim::*;
// pub use {algebra::*, prim::*};

pub(crate) mod all {
    // feature-gated
    #[doc(inline)]
    #[cfg(feature = "fig")]
    pub use super::prim::*;
    // pub use super::{algebra::*, prim::*};
}
