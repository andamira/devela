// devela/src/sys/os/term/_.rs
//
#![doc = crate::_DOC_SYS_OS_TERM!()] // public
#![doc = crate::_doc!(modules: crate::sys::os; term: ansi, grid)]
#![doc = crate::_doc!(flat:"sys")]
#![doc = crate::_doc!(hr)]
//

crate::mods_in! {
    pub mod_ ansi; // Ansi, AnsiColor3, AnsiColor8
        mod_ backend; // TermLinux
        mod_ cap; // TermCaps
        #[cfg(feature = "event")]
        mod_ event; // TermInputParser
    pub mod_ grid; // Terminal cell elements, grids, and composition
        mod_ line; // TermLineMode
        mod metric; // TermSize
        mod pen; // TermPen
        mod_ render; // TermRenderer
        mod session; // TermSession, TermPollPolicy, TermMode
        #[cfg(feature = "ui")]
        mod_ ui; // UI realizations for the terminal
        // mod_ vterm; // TODO
}
crate::mods_out! { // _mods, _pub_mods, _crate_internals
    _mods {
        pub use super::{
            backend::_all::*,
            cap::_all::*,
            line::_all::*,
            metric::TermSize,
            pen::TermPen,
            render::_all::*,
            session::*,
        };
        #[cfg(feature = "event")]
        pub use super::event::_all::*;
        #[cfg(feature = "ui")]
        pub use super::ui::_all::*;
    }
    _pub_mods {
        pub use super::{
            ansi::_all::*,
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
