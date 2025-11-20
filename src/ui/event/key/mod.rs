// devela::ui::event::key
//
//! Defines [`EventKey`], [`KeyState`].
//
// TOC
// - struct EventKey
// - enum KeyState
// - mod ffi
//   - struct EventKeyFfi
// - tests

#[cfg(test)]
mod tests;

mod dif; // KeyMod, KeyMods
mod event; // EventKey, EventKeyFfi, KeyState
mod key; // Key
mod media; // KeyMedia, KeyMod, KeyMods
mod pad; // KeyPad

#[cfg(ffi··)]
#[cfg_attr(nightly_doc, doc(cfg(ffi··)))]
mod ffi; // KeyFfi

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            dif::*,
            event::*,
            key::*,
            media::*,
            pad::*,
        };
        #[cfg(ffi··)]
        pub use super::ffi::*;
    }
}
