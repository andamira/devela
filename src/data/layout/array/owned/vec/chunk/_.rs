// devela/src/data/layout/array/owned/vec/chunk/_.rs
//
//!
//

crate::mods_in! {
    #[cfg(test)]
    mod _test;

    mod define; // VecChunk
}
crate::mods_out! { // _mods
    _mods {
        pub use super::{
            define::VecChunk,
        };
    }
}
