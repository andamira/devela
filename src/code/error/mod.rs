// devela::code::error
//
#![doc = crate::_DOC_CODE_ERROR!()]
//!
//! Re-exports the error and result types defined in other modules and crates.
//!
#![doc = crate::_doc!(extends: backtrace, error)]
//

mod _reexport_core; // SYMLINK to /libs/base_core/src/code/error/_reexport.rs
mod _reexport_std; // SYMLINK TO /libs/base_std/src/code/error/_reexport.rs

crate::structural_mods! { // _reexports
    _reexports {
        pub use super::{
            _reexport_core::*,
            _reexport_std::*,
        };

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

        /* intra-crate */

        #[doc(inline)]
        #[cfg(feature = "image")]
        pub use crate::media::image::{ImageError, ImageResult};

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
