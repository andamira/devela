// devela/src/sys/os/term/ui/mod.rs
//
//! UI realizations for the terminal.
//

mod cell; // TermCellUi

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            cell::*,
        };
    }
}
