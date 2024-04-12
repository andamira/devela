// devela::ui
//
//! User interface fundamentals and minimal implementations.
//

// safety:
#![cfg_attr(feature = "safe_ui", forbid(unsafe_code))]

mod error;
pub use error::*;

mod cap;
mod service;
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
    pub use super::{cap::*, service::*};

    #[doc(inline)]
    #[allow(unused_imports)]
    #[cfg(feature = "ui_term")]
    pub use super::term::all::*;
}
