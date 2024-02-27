// devela::ui::term
//
//! Terminal functionality.
//

// safety:
#![cfg_attr(feature = "safe_ui_term", forbid(unsafe_code))]

mod ansi;

// re-export private sub-modules
pub use ansi::*;

pub(crate) mod all {
    #[doc(inline)]
    pub use super::ansi::*;
}
