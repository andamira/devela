// devela/src/sys/os/browser/web/event/key/mod.rs
//
//!
//

mod key; // WebEventKey
mod location; // WebKeyLocation, impl:KeyMod,KeyMods

mod media; // impl:KeyMedia
mod pad; // impl:KeyPad

crate::mods_out! { // _mods
    _mods {
        pub use super::{
            key::*,
            location::*,
        };
    }
}
