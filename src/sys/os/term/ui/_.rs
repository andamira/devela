// devela/src/sys/os/term/ui/_.rs
//
//! UI realizations for the terminal.
//

crate::mods_in! {
    mod cell; // TermCellUi
}
crate::mods_out! { // _mods
    _mods {
        pub use super::{
            cell::TermCellUi,
        };
    }
}
