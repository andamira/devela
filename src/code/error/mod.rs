// devela::code::error
//
#![doc = crate::_DOC_CODE_ERROR!()] // public
#![doc = crate::_doc!(modules: crate::code; error)]
#![doc = crate::_doc!(flat:"code")]
#![doc = crate::_doc!(extends: backtrace, error)]
//!
//! Re-exports the error and result types defined in other modules and crates.
//!
//

mod _reexport_core; // SYMLINK to /crates/base/core/src/code/error/_reexport.rs
#[cfg(feature = "std")]
mod _reexport_std;

crate::structural_mods! { // _reexports
    _reexports {
        pub use super::_reexport_core::*;
        #[cfg(feature = "std")]
        pub use super::_reexport_std::*;

        #[doc(inline)]
        pub use devela_base_core::{
            define_error,
            // individual:
            FailedErrorConversion,
            InvalidValue,
            NotImplemented,
            NotSupported,
            // composite:
            NotAvailable,
        };

        /* intra-crate */ // RETHINK

        #[doc(inline)]
        #[cfg(feature = "image")]
        pub use crate::media::visual::image::{ImageError, ImageResult};

        /* num */
        #[doc(inline)]
        pub use crate::num::{NumError, NumResult};

        /* sys */
        #[doc(inline)]
        #[cfg(feature = "io")]
        pub use crate::sys::io::{IoError, IoErrorKind, IoResult};

        #[doc(inline)]
        #[cfg(text··)]
        pub use crate::text::{TextError, TextResult};
    }
}
