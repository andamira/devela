// devela/src/error/data/_.rs
//
//! Data-related errors.
//

crate::mods_in! {
    mod capacity;
    mod other;
}
crate::mods_out! { // _mods
    _mods {
        pub use super::{
            capacity::*,
            other::*,
        };
    }
}
