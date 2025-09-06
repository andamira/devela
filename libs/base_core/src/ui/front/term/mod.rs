// devela_base_core::ui::front::term
//
//! UI terminal functionality.
//

mod size; // TermSize
mod ansi; // Ansi, AnsiColor3b, AnsiColor8b

crate::structural_mods! { // _mods
    _mods {
        pub use super::size::*;
        pub use super::ansi::_all::*;
    }
}
