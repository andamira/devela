// devela::sys::os::term
//
#![doc = crate::_DOC_SYS_OS_TERM!()] // public
#![doc = crate::_doc!(modules: crate::sys::os; term)]
#![doc = crate::_doc!(flat:"sys")]
#![doc = crate::_doc!(hr)]
//
// THINK: .

mod ansi; // Ansi, AnsiColor3, AnsiColor8
// mod backend; // WIP
// mod cap; // WIP
#[cfg(feature = "event")]
mod event; // TermInputParser WIP
mod metric; // TermSize
// #[cfg(feature = "term")]
// mod render; // TermRenderer WIP

crate::structural_mods! { // _mods, _crate_internals
    _mods {
        pub use super::{
            // cap::*,
            metric::*,
        };
        pub use super::{
            ansi::_all::*,
            // backend::_all::*,
            // render::_all::*,
        };
        #[cfg(feature = "event")]
        pub use super::event::_all::*;
    }
    _crate_internals {
        pub use super::ansi::_crate_internals::*;
    }
}
