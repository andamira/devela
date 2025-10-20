// devela_base_core::ui::front
//
//! UI frontends.
//

#[cfg(feature = "term")]
#[cfg_attr(nightly_doc, doc(cfg(feature = "term")))]
pub mod term;

crate::structural_mods! { // _pub_mods, _crate_internals
    _pub_mods {
        #[cfg(feature = "term")]
        pub use super::term::_all::*;
    }
    _crate_internals {
        #[cfg(feature = "term")]
        pub use super::term::_crate_internals::*;
    }
}
