// devela::mix::color
//
//! Chromatic functionality.
//

/* feature-gated, non-public modules */

#[cfg(feature = "mix")]
mod error;
#[cfg(feature = "mix")]
mod fns;
// #[cfg(feature = "mix")]
// mod rgb;
#[cfg(feature = "mix")]
mod r#trait;

#[cfg(feature = "mix")]
pub use {error::*, fns::*, r#trait::* /*rgb::* */};

pub(crate) mod all {
    // feature-gated
    #[doc(inline)]
    #[cfg(feature = "mix")]
    pub use super::{error::*, fns::*, r#trait::* /*rgb::* */};
}
