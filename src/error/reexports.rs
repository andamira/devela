// devela::error::reexports
//
//! Reexported items.
//

#![allow(unused_imports)]

use crate::reexport;

pub use crate_errors::*;
pub(crate) mod crate_errors {
    /* rend */
    #[doc(inline)]
    #[cfg(feature = "audio")]
    pub use crate::rend::audio::{AudioError, AudioResult};
    #[doc(inline)]
    #[cfg(feature = "color")]
    pub use crate::rend::color::{ColorError, ColorResult};
    #[doc(inline)]
    #[cfg(feature = "draw")]
    pub use crate::rend::draw::{DrawError, DrawResult};
    #[doc(inline)]
    #[cfg(feature = "font")]
    pub use crate::rend::font::{FontError, FontResult};
    #[doc(inline)]
    #[cfg(feature = "image")]
    pub use crate::rend::image::{ImageError, ImageResult};
    #[doc(inline)]
    #[cfg(feature = "layout")]
    pub use crate::rend::layout::{LayoutError, LayoutResult};
    #[doc(inline)]
    #[cfg(_rend_·)]
    pub use crate::rend::{RendError, RendResult};

    /* sys */
    #[doc(inline)]
    #[cfg(feature = "io")]
    pub use crate::sys::io::{IoError, IoErrorKind, IoResult};
    #[doc(inline)]
    #[cfg(feature = "time")]
    pub use crate::sys::time::{TimeError, TimeResult};

    /* other */
    #[doc(inline)]
    pub use crate::{
        data::{DataError, DataResult},
        num::{NumError, NumResult},
        text::{TextError, TextResult},
    };
}

/* `core` re-exports */

reexport! { rust: core,
    doc: "Causes compilation to fail with the given error message when encountered.",
    compile_error
}
reexport! { rust: core::error,
    doc: "A trait representing the basic expectations for error values.",
    Error
}
reexport! { rust: core::result,
    doc: "A type that represents either success ([`Ok`]) or failure ([`Err`]).",
    Result
}

#[cfg(feature = "std")]
pub use std::*;
#[cfg(feature = "std")]
mod std {
    use super::reexport;

    reexport! { rust: std::backtrace,
        doc: "A captured OS thread stack backtrace.",
        Backtrace
    }
    reexport! { rust: std::backtrace,
        doc: "The current status of a backtrace.",
        BacktraceStatus
    }
}
