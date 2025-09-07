// devela::code::error::reexports
//
//!
//

crate::mod_path!(+pub _c "../../../libs/base_core/src/code/error/reexports.rs");
crate::mod_path!(std +pub _s "../../../libs/base_std/src/code/error/reexports.rs");

#[doc(inline)] #[rustfmt::skip]
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

pub use crate_errors::*;
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
