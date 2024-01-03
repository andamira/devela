// devela::ui
//
//! User interface functionality.
//

/* contains always compiled items */

#[cfg(feature = "ui_term")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "ui_term")))]
pub mod term;

/* re-exports */

// re-export public sub-modules
#[doc(no_inline)]
#[cfg(feature = "ui_term")]
pub use term::all::*;

pub(crate) mod all {
    #[doc(inline)]
    #[cfg(feature = "ui_term")]
    pub use super::term::all::*;
}
