// devela/src/sys/os/browser/web/event/key/key/mod.rs

mod key; // WebEventKey

mod compact;
mod impls;

crate::mods_out! { // _mods
    _mods {
        pub use super::{
            key::*,
        };
    }
}
