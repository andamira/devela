// devela/src/sys/os/term/ui/mod.rs
//
//! UI realizations for the terminal.
//

mod cell; // TermCellUi

crate::mods_out! { // _mods
    _mods {
        pub use super::{
            cell::*,
        };
    }
}
