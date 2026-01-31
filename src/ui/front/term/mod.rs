// devela::ui::front::term
//
#![doc = crate::_DOC_UI_FRONT_TERM!()] // public
#![doc = crate::_doc!(modules: crate::ui; front)]
#![doc = crate::_doc!(flat:"ui")]
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
        pub use devela_base_core::ui::front::term::TermSize;

        #[doc(inline)]
        #[cfg(feature = "term")]
        pub use devela_base_core::ui::front::term::{
            Ansi, AnsiColor, AnsiColor3, AnsiColor8,
        };
    }
}
