// devela::rend::color
//
//! Chromatic functionality.
//

#[cfg(feature = "rend_color")]
mod error;
#[cfg(feature = "rend_color")]
mod fns;
// #[cfg(feature = "rend_color")]
// mod rgb;
#[cfg(feature = "rend_color")]
mod r#trait;
#[cfg(feature = "rend_color")]
pub use {error::*, fns::*, r#trait::* /*rgb::* */};

pub(crate) mod all {
    #[doc(inline)]
    #[cfg(feature = "rend_color")]
    pub use super::{error::*, fns::*, r#trait::* /*rgb::* */};
}
