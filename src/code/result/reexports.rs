// devela::code::result::reexports
//
//! Reexported items.
//

#![allow(unused_imports)]

use crate::reexport;

pub use crate_errors::*;
pub(crate) mod crate_errors {
    /* data */
    #[doc(inline)]
    #[cfg(feature = "data")]
    pub use crate::data::{DataError, DataResult};

    /* media */
    #[doc(inline)]
    #[cfg(feature = "audio")]
    pub use crate::media::audio::{AudioError, AudioResult};
    #[doc(inline)]
    #[cfg(feature = "color")]
    pub use crate::media::color::{ColorError, ColorResult};
    #[doc(inline)]
    #[cfg(feature = "draw")]
    pub use crate::media::draw::{DrawError, DrawResult};
    #[doc(inline)]
    #[cfg(feature = "font")]
    pub use crate::media::font::{FontError, FontResult};
    #[doc(inline)]
    #[cfg(feature = "image")]
    pub use crate::media::image::{ImageError, ImageResult};
    #[doc(inline)]
    #[cfg(_media_·)]
    pub use crate::media::{MediaError, MediaResult};

    /* phys */
    #[doc(inline)]
    #[cfg(feature = "time")]
    pub use crate::phys::time::{TimeError, TimeResult};

    /* sys */
    #[doc(inline)]
    #[cfg(feature = "io")]
    pub use crate::sys::io::{IoError, IoErrorKind, IoResult};

    /* ui */
    #[doc(inline)]
    #[cfg(feature = "layout")]
    pub use crate::ui::layout::{LayoutError, LayoutResult};

    /* other */
    #[doc(inline)]
    pub use crate::{
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
