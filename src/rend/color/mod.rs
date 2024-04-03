// devela::rend::color
//
//! Chromatic functionality.
//

/* feature-gated */

#[cfg(feature = "rend")]
mod error;
#[cfg(feature = "rend")]
mod fns;
// #[cfg(feature = "rend")]
// mod rgb;
#[cfg(feature = "rend")]
mod r#trait;
#[cfg(feature = "rend")]
pub use {error::*, fns::*, r#trait::* /*rgb::* */};

pub(crate) mod all {
    // feature-gated
    #[doc(inline)]
    #[cfg(feature = "rend")]
    pub use super::{error::*, fns::*, r#trait::* /*rgb::* */};
}
