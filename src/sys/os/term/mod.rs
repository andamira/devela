// devela::sys::os::term
//
#![doc = crate::_DOC_SYS_OS_TERM!()] // public
#![doc = crate::_doc!(modules: crate::sys::os; term)]
#![doc = crate::_doc!(flat:"sys")]
#![doc = crate::_doc!(hr)]
//

#[cfg(feature = "term")]
#[cfg_attr(nightly_doc, doc(cfg(feature = "term")))]
mod ansi;

crate::structural_mods! { // _mods, _reexports
    _mods {
        #[cfg(feature = "term")]
        pub use super::ansi::_all::*;
    }
    _reexports {
        #[doc(inline)]
        pub use devela_base_core::sys::os::term::TermSize;

        #[doc(inline)]
        #[cfg(feature = "term")]
        pub use devela_base_core::sys::os::term::{
            Ansi, AnsiColor, AnsiColor3, AnsiColor8,
        };
    }
}
