// devela/src/num/grain/cast/traits/_.rs
//
//! Helpers for converting between primitives.
//

crate::mods_in! {
    mod cast; // PrimCast
    mod join; // PrimJoin
    mod split; // PrimSplit
}
crate::mods_out! { // _mods
    _mods {
        pub use super::{
            cast::PrimCast,
            join::PrimJoin,
            split::PrimSplit,
        };
    }
}
