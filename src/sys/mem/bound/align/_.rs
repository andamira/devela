// devela/src/sys/mem/bound/align/_.rs
//
//! Memory alignment bounds.
//

crate::mods_in! {
    mod aligned;
    mod cache;
}
crate::mods_out! { // _mods
    _mods {
        pub use super::{
            aligned::MemAligned,
            cache::CacheAlign,
        };
    }
}
