// revela::ui::front
//
//! UI frontends.
//

#[cfg(feature = "term")]
#[cfg_attr(nightly_doc, doc(cfg(feature = "term")))]
pub mod term;

crate::structural_mods! { // _pub_mods
    _pub_mods {
        #[cfg(feature = "term")]
        pub use super::term::_all::*;
    }
}
