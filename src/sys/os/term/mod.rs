// devela/src/sys/os/term/mod.rs
//
#![doc = crate::_DOC_SYS_OS_TERM!()] // public
#![doc = crate::_doc!(modules: crate::sys::os; term: grid)]
#![doc = crate::_doc!(flat:"sys")]
#![doc = crate::_doc!(hr)]
//

mod ansi; // Ansi, AnsiColor3, AnsiColor8
mod backend; // TermLinux
mod cap; // TermCaps
#[cfg(feature = "event")]
mod event; // TermInputParser
pub mod grid; // Terminal cell elements, grids, and composition
mod line; // TermLineMode
mod metric; // TermSize
mod pen; // TermPen
mod render; // TermRenderer
mod session; // TermSession, TermPollPolicy, TermMode

crate::structural_mods! { // _mods, _pub_mods, _crate_internals
    _mods {
        pub use super::{
            ansi::_all::*,
            backend::_all::*,
            cap::_all::*,
            line::_all::*,
            metric::*,
            pen::*,
            render::_all::*,
            session::*,
        };
        #[cfg(feature = "event")]
        pub use super::event::_all::*;
    }
    _pub_mods {
        pub use super::{
            grid::_all::*,
        };
    }
    _crate_internals {
        pub(crate) use super::{
            ansi::_crate_internals::*,
            backend::_crate_internals::*,
        };
        #[cfg(feature = "event")]
        pub(crate) use super::event::_crate_internals::*;
    }
}
