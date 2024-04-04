// devela::ui
//
//! User interface fundamentals and minimal implementations.
//

// safety:
#![cfg_attr(feature = "safe_ui", forbid(unsafe_code))]

mod error;
pub use error::*;

#[cfg(feature = "data_bit")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "ui_service")))]
mod cap;
#[cfg(feature = "data_bit")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "ui_service")))]
mod service;
#[cfg(feature = "data_bit")]
pub use {cap::*, service::*};

#[cfg(feature = "ui_term")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "ui_term")))]
pub mod term;
#[doc(no_inline)]
#[cfg(feature = "ui_term")]
pub use term::all::*;

pub(crate) mod all {
    #[doc(inline)]
    #[allow(unused_imports)]
    pub use super::error::*;

    #[doc(inline)]
    #[allow(unused_imports)]
    #[cfg(feature = "data_bit")]
    pub use super::{cap::*, service::*};

    #[doc(inline)]
    #[allow(unused_imports)]
    #[cfg(feature = "ui_term")]
    pub use super::term::all::*;
}
