// devela::ui::term
//
//! Terminal functionality.
//

mod ansi;

// re-export private sub-modules
#[cfg(feature = "ui_term")]
pub use ansi::*;

pub(crate) mod all {
    #[doc(inline)]
    pub use super::ansi::*;
}
