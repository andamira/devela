// devela/src/sys/os/browser/web/event/key/key/mod.rs

mod key; // WebEventKey

mod compact;
mod impls;

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            key::*,
        };
    }
}
