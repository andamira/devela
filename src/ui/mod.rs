// devela::ui
//
//! User interface fundamentals and minimal implementations.
//

// safety:
#![cfg_attr(feature = "safe_ui", forbid(unsafe_code))]

/* modules */

// feature-gated, public
#[cfg(feature = "ui_term")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "ui_term")))]
pub mod term;

/* re-exports */

// feature-gated, public
#[doc(no_inline)]
#[cfg(feature = "ui_term")]
pub use term::all::*;

pub(crate) mod all {
    // feature-gated
    #[doc(inline)]
    #[cfg(feature = "ui_term")]
    pub use super::term::all::*;
}
