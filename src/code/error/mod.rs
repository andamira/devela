// devela::code::error
//
#![doc = crate::_DOC_CODE_ERROR!()] // public
#![doc = crate::_doc!(modules: crate::code; error)]
#![doc = crate::_doc!(flat:"code")]
#![doc = crate::_doc!(extends: backtrace, error)]
//!
//! Re-exports the error and result types defined in other modules and crates.
//

mod _reexport_core;
#[cfg(feature = "std")]
mod _reexport_std;

// mod context; // ContextualError WIP
mod define; // define_error!
mod errors; // general errors

crate::structural_mods! { // _mods, _reexports
    _mods {
        #[doc(inline)]
        pub use super::{
            // context::*,
            define::define_error,
            errors::*,
        };
    }
    _reexports {
        pub use super::_reexport_core::*;
        #[cfg(feature = "std")]
        pub use super::_reexport_std::*;

        /* intra-crate */ // RETHINK

        #[doc(inline)]
        #[cfg(feature = "image")]
        pub use crate::media::visual::image::{ImageError, ImageResult};

        #[doc(inline)]
        pub use crate::num::{NumError, NumResult};

        #[doc(inline)]
        #[cfg(feature = "io")]
        pub use crate::sys::io::{IoError, IoErrorKind, IoResult};

        #[doc(inline)]
        #[cfg(text··)]
        pub use crate::text::{TextError, TextResult};
    }
}
