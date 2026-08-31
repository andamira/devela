// devela/src/data/access/cursor/byte/_.rs
//
//! Cursor-based access over ordered byte regions.
//

crate::mods_in! {
    #[cfg(test)]
    mod _test;

    mod define; // ByteCursor (+ impl common)
    mod read;
    mod write;
}
crate::mods_out! { // _mods
    _mods {
        pub use super::{
            define::*,
        };
    }
}
