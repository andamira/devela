// devela_base_core::sys::os::term
//
#![doc = crate::_DOC_SYS_OS_TERM!()] // public
#![doc = crate::_doc!(modules: crate::sys::os; term)]
#![doc = crate::_doc!(flat:"sys")]
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
