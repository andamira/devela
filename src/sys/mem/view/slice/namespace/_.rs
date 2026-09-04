// devela/src/sys/mem/view/slice/namespace/_.rs
//
//! Defines the [`Slice`] namespace.
//

crate::mods_in! {
    #[cfg(test)]
    mod _test;

    // impls
    mod core;
    mod range;
    mod take;
    mod split;
    mod chunk;
    mod bytes;
    mod eq;

    mod define; // Slice
    mod r#macro; // slice!
}
crate::mods_out! { // _mods
    _mods {
        pub use super::{
            define::Slice,
            r#macro::slice,
        };
    }
}
