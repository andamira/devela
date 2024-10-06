// devela::gfx
//
//! Graphics related functionality.
#![doc = crate::code::doc_!(modules: crate; gfx: image)]
#![doc = crate::code::doc_!(newline)]
//

// safety:
#![cfg_attr(feature = "safe_gfx", forbid(unsafe_code))]

mod error;
pub use error::*;

// pub mod color;
// pub mod draw;
// pub mod fonts;

#[cfg(feature = "gfx_image")]
crate::code::items! {
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "gfx_image")))]
    pub mod image;
    #[doc(no_inline)]
    #[allow(unused_imports)]
    pub use {error::*, image::*};
}

pub(crate) mod all {
    #[doc(inline)]
    #[allow(unused_imports)]
    pub use super::error::*;

    #[doc(inline)]
    #[allow(unused_imports)]
    #[cfg(feature = "gfx_image")]
    pub use super::image::all::*;
}
