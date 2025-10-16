// devela_base_core::ui::front::term
//
//! UI terminal functionality.
//

mod size; // TermSize
mod ansi; // Ansi, AnsiColor3, AnsiColor8

crate::structural_mods! { // _mods, _crate_internals
    _mods {
        pub use super::size::*;
        pub use super::ansi::_all::*;
    }
    _crate_internals {
        pub use super::ansi::_crate_internals::*;
    }
}
