// devela::num
//
//! Numeric types, extends [`core::num`].
//!
//! It also reexports the `NonZero*` types from `core`.
//

/* always compiled for internal use */

mod always_fns;
mod non_specific;
#[allow(unused)]
#[cfg(not(feature = "num"))]
pub use {always_fns::*, non_specific::*};

/* only compiled with the `mem` feature */

#[cfg(feature = "num")]
mod non_range;
#[cfg(feature = "num")]
mod range;
#[cfg(feature = "num")]
mod r#trait;

#[cfg(all(feature = "num", test))]
mod tests;

/* re-exports */

#[cfg(feature = "num")]
mod reexports;

#[cfg(feature = "num")]
pub use all::*;
#[cfg(feature = "num")]
pub(crate) mod all {
    #[doc(inline)]
    pub use super::{
        always_fns::*, non_range::*, non_specific::*, r#trait::*, range::*, reexports::*, fsize,
    };
}

/* misc */

/// An alias for a pointer-sized floating-point primitive.
#[cfg(target_pointer_width = "32")]
#[allow(non_camel_case_types)]
pub type fsize = f32;

/// An alias for a pointer-sized floating-point primitive.
#[cfg(target_pointer_width = "64")]
#[allow(non_camel_case_types)]
#[cfg_attr(
    feature = "nightly",
    doc(cfg(any(target_pointer_width = "32", target_pointer_width = "64")))
)]
pub type fsize = f64;
