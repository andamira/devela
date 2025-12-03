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

mod dead; // KeyDead
mod event; // EventKey, EventKeyFfi
mod key; // Key
mod media; // KeyMedia, KeyMod, KeyMods
mod mods; // KeyMod, KeyMods
mod pad; // KeyPad
mod state; // KeyState

#[cfg(ffi··)]
#[cfg_attr(nightly_doc, doc(cfg(ffi··)))]
mod ffi; // KeyFfi

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            dead::*,
            event::*,
            key::*,
            media::*,
            mods::*,
            pad::*,
            state::*,
        };
        #[cfg(ffi··)]
        pub use super::ffi::*;
    }
}
