// devela::ui
//
//! User interface fundamentals and minimal implementations.
//

// safety:
#![cfg_attr(feature = "safe_ui", forbid(unsafe_code))]

/* always compiled, non-public modules */

/* feature-gated, public modules */

#[cfg(feature = "ui_term")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "ui_term")))]
pub mod term;

#[doc(no_inline)]
#[cfg(feature = "ui_term")]
pub use term::all::*;

pub(crate) mod all {
    // feature-gated
    #[doc(inline)]
    #[cfg(feature = "ui_term")]
    pub use super::term::all::*;
}
