// devela/src/sys/os/browser/web/event/key/_.rs
//
//!
//

crate::mods_in! {
    mod_ key_; // WebEventKey WAIT:circular-module
    mod location; // WebKeyLocation, impl:KeyMod,KeyMods

    mod media; // impl:KeyMedia
    mod pad; // impl:KeyPad
}
crate::mods_out! { // _mods
    _mods {
        pub use super::{
            key_::_all::*,
            location::*,
        };
    }
}
