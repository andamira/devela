// devela_base_core::ui::front::term
//
#![doc = crate::_DOC_UI_FRONT_TERM!()] // public
#![doc = crate::_doc!(modules: crate::ui; front)]
#![doc = crate::_doc!(flat:"ui")]
#![doc = crate::_doc!(hr)]
//

mod size; // TermSize

#[cfg(feature = "term")]
#[cfg_attr(nightly_doc, doc(cfg(feature = "term")))]
mod ansi; // Ansi, AnsiColor3, AnsiColor8

crate::structural_mods! { // _mods, _crate_internals
    _mods {
        pub use super::size::*;
        #[cfg(feature = "term")]
        pub use super::ansi::_all::*;
    }
    _crate_internals {
        #[cfg(feature = "term")]
        pub use super::ansi::_crate_internals::*;
    }
}
