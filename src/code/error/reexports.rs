// devela::code::error::reexports
//
//! Reexported error-related items.
//

#![allow(unused_imports)]

use crate::reexport;
reexport! { rust: core::error,
    doc: "A trait representing the basic expectations for error values.",
    Error
}

reexport! { rust: core,
    doc: "Causes compilation to fail with the given error message when encountered.",
    compile_error
}

reexport! { rust: std::backtrace,
    doc: "A captured OS thread stack backtrace.",
    Backtrace
}
reexport! { rust: std::backtrace,
    doc: "The current status of a backtrace.",
    BacktraceStatus
}

#[cfg(feature = "error")]
pub use crate_errors::*;
#[cfg(feature = "error")]
pub(crate) mod crate_errors {
    /* data */
    #[doc(inline)]
    #[cfg(data··)]
    pub use crate::{DataError, DataResult};

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
