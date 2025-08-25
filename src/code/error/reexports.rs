// devela::code::error::reexports
//
//!
//

use crate::_reexport_from;

// from workspace base
_reexport_from!("../../../libs/base/src/code/error/reexports.rs", _c);
_reexport_from!(std "../../../libs/base_std/src/code/error/reexports.rs", _s);

#[doc(inline)] #[rustfmt::skip]
pub use devela_base::{
    // individual:
    FailedErrorConversion,
    InvalidValue,
    NotImplemented,
    NotSupported,
    // composite:
    NotAvailable,
};

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
