// devela/src/text/str/array/_.rs
//
//!
//

crate::mods_in! {
    mod nonul; // StringNonNul
    mod u; // StringU8, StringU16
}
crate::mods_out! { // _mods
    _mods {
        pub use super::{
            nonul::*,
            u::*,
        };
    }
}
