// devela/src/data/value/kind/mod.rs
//

mod four; // ValueKind4
mod kind; // ValueKind
// mod set; // ValueKindSet WIP

crate::structural_mods! { // _mods, _pub_mods
    _mods {
        pub use super::{
            four::*,
            kind::*,
            // set::*,
        };
    }
}
