// devela::error::reexports
//
//! Reexported items.
//

#![allow(unused_imports)]

use crate::code::reexport;

/* from other modules */

#[doc(inline)]
#[cfg(feature = "audio")]
pub use crate::rend::{AudioError, AudioResult};
#[doc(inline)]
#[cfg(feature = "color")]
pub use crate::rend::{ColorError, ColorResult};
#[doc(inline)]
#[cfg(feature = "draw")]
pub use crate::rend::{DrawError, DrawResult};
#[doc(inline)]
#[cfg(feature = "font")]
pub use crate::rend::{FontError, FontResult};
#[doc(inline)]
#[cfg(feature = "image")]
pub use crate::rend::{ImageError, ImageResult};
#[doc(inline)]
#[cfg(feature = "layout")]
pub use crate::rend::{LayoutError, LayoutResult};
#[doc(inline)]
#[cfg(_rend_·)]
pub use crate::rend::{RendError, RendResult};
#[doc(inline)]
#[cfg(feature = "time")]
pub use crate::sys::time::{TimeError, TimeResult};
#[doc(inline)]
pub use crate::{
    data::{DataError, DataResult},
    num::{NumError, NumResult},
    sys::{IoError, IoErrorKind, IoResult},
    text::{TextError, TextResult},
};

/* `core` re-exports */

reexport! { rust: core,
    doc: "Causes compilation to fail with the given error message when encountered.",
    compile_error
}

reexport! { rust: core::error,
    doc: "A trait representing the basic expectations for error values.",
    Error
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
