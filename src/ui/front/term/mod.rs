// devela::ui::front::term
//
//! Terminal functionality.
//

mod size;
mod ansi;

crate::structural_mods! { // _mods
    _mods {
        pub use super::size::*;
        pub use super::ansi::*;
    }
}
