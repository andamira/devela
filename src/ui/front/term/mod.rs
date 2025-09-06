// devela::ui::front::term
//
//! Terminal functionality.
//

mod ansi;

crate::structural_mods! { // _mods
    _mods {
        pub use super::ansi::_all::*;

        // re-exports
        #[doc(inline)]
        pub use devela_base_core::ui::front::term::{Ansi, AnsiColor3b, AnsiColor8b, TermSize};
    }
}
