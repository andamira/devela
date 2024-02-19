// devela::fig
//
//! Figures, geometric types and operations.
//

// safety:
#![cfg_attr(feature = "safe_fig", forbid(unsafe_code))]

/* feature-gated, private modules */

// #[cfg(feature = "fig")]
// mod x;
//
// #[doc(no_inline)]
// #[cfg(feature = "fig")]
// pub use x::*;

pub(crate) mod all {
    // // feature-gated
    // #[doc(inline)]
    // #[cfg(feature = "fig")]
    // pub use super::x::*;
}
