// devela/src/sys/device/mod.rs
//
#![doc = crate::_DOC_SYS_DEVICE!()] // public
#![doc = crate::_doc!(modules: crate::sys; device: audio, display)] // gpu, midi
#![doc = crate::_doc!(flat:"sys")]
#![doc = crate::_doc!(hr)]
#![doc = crate::_QUO_SYS_DEVICE!()]
//!
//! Usable runtime device interfaces exposed by the host.
//!
//! This includes physical, virtual, OS-mediated, server-backed, and sandboxed
//! endpoints such as audio streams, display backends, and input/output devices.
//! Items are classified here by the live capability they provide to a running
//! program, not by the underlying hardware or protocol.
//

pub mod audio; // {alsa}
pub mod display; // {x11}
// pub mod midi; //

crate::structural_mods! { // _pub_mods, _crate_internals
    _pub_mods {
        pub use super::{
            audio::_all::*,
            display::_all::*,
            // midi::_all::*,
        };
    }
    _crate_internals {
        pub(crate) use super::{
            audio::_crate_internals::*,
            display::_crate_internals::*,
        };
    }
}
