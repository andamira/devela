// devela::code::error::reexports
//
//! Reexported error-related items.
//

#![allow(unused_imports)]

use crate::_reexport;

#[doc = crate::TAG_DEVELA_BASE!()]
#[doc(inline)] #[rustfmt::skip]
pub use devela_base::{
    FailedErrorConversion,
    NotAvailable,
        NotImplemented,
        NotSupported,

    code::util::_tags::*, // _workspace_private
};

_reexport! { rust: core::error,
    doc: "A trait representing the basic expectations for error values.",
    Error
}
_reexport! { rust: core,
    doc: "Causes compilation to fail with the given error message when encountered.",
    compile_error
}

_reexport! { rust: std::backtrace,
    doc: "A captured OS thread stack backtrace.",
    Backtrace
}
_reexport! { rust: std::backtrace,
    doc: "The current status of a backtrace.",
    BacktraceStatus
}

#[cfg(feature = "error")]
pub use crate_errors::*;
#[cfg(feature = "error")]
pub(crate) mod crate_errors {
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
