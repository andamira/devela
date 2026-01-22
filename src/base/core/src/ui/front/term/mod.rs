// devela_base_core::ui::front::term
//
//! UI terminal functionality.
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
