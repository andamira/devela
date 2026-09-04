// devela/src/num/grain/cast/namespace/_.rs
//
//! Helpers for converting between primitives.
//

crate::mods_in! {
    mod define; // Cast

    // impls
    mod cast;
    mod join;
    mod split;
}
crate::mods_out! { // _mods
    _mods {
        pub use super::{
            define::Cast,
        };
    }
}
