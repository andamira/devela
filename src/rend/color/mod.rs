// devela::rend::color
//
//! Chromatic functionality.
//

// safety:
#![cfg_attr(feature = "safe_color", forbid(unsafe_code))]

mod base;
mod error;
mod namespace;
#[allow(unused_imports)]
pub use {base::*, error::*, namespace::*};

pub(crate) mod all {
    #[doc(inline)]
    #[allow(unused_imports)]
    pub use super::{base::*, error::*, namespace::*};
}
