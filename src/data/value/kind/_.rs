// devela/src/data/value/kind/_.rs
//
//!
//

crate::mods_in! {
    mod four; // ValueKind4
    mod kind; // ValueKind
    // mod set; // ValueKindSet WIP
}
crate::mods_out! { // _mods
    _mods {
        pub use super::{
            four::*,
            kind::*,
            // set::*,
        };
    }
}
