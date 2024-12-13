// devela::code::result::error::reexports
//
//! Reexported error-related items.
//

#![allow(unused_imports)]

use crate::reexport;
reexport! { rust: core::error,
    doc: "A trait representing the basic expectations for error values.",
    Error
}

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

    /* num */
    #[doc(inline)]
    pub use crate::num::{NumError, NumResult};

    /* phys */
    #[doc(inline)]
    #[cfg(feature = "time")]
    pub use crate::phys::time::{TimeError, TimeResult};

    /* sys */
    #[doc(inline)]
    #[cfg(feature = "io")]
    pub use crate::sys::io::{IoError, IoErrorKind, IoResult};

    #[doc(inline)]
    #[cfg(feature = "text")]
    pub use crate::text::{TextError, TextResult};

    /* ui */
    #[doc(inline)]
    #[cfg(feature = "layout")]
    pub use crate::ui::layout::{LayoutError, LayoutResult};
}
