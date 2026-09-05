// devela/src/sys/os/term/render/_.rs
//
//! Terminal byte-frame rendering.
//

crate::mods_in! {
    #[cfg(test)]
    mod _test;

    mod define; // TermRenderer

    // impls
    mod core_io;
    mod shared;
    mod exclusive;
    #[cfg(feature = "alloc")]
    mod owned;
    mod grid;
}
crate::mods_out! { // _mods
    _mods {
        pub use super::{
            define::TermRenderer,
        };
    }
}
