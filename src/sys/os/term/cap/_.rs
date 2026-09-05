// devela/src/sys/os/term/cap/_.rs
//
//! Terminal capabilities.
//

crate::mods_in! {
    mod field; // TermCaps
    mod list; // TermCap
}
crate::mods_out! { // _mods
    _mods {
        pub use super::{
            field::*,
            list::*,
        };
    }
}
