// devela/src/ui/text/input/_.rs
//
//! Interactive text state for input surfaces.
//

crate::mods_in! {
    #[cfg(test)]
    mod _test;

    mod define; // TextInput[Config|Action|Reject|View]
    // impls
    mod _helper;
    mod core;
    #[cfg(feature = "alloc")]
    mod alloc;

    #[cfg(feature = "event")]
    mod event; // WIP TextInputKeymap[Preset]
}
crate::mods_out! { // _mods
    _mods {
        pub use super::{
            define::*,
        };
        #[cfg(feature = "event")]
        pub use super::event::*;
    }
}
