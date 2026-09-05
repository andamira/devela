// devela/src/sys/device/_.rs
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

crate::mods_in! {
    pub mod_ audio; // {alsa}
    pub mod_ display; // {x11}
    // pub mod_ gpu; //
    // pub mod_ midi; //
}
crate::mods_out! { // _pub_mods, _crate_internals
    _pub_mods {
        pub use super::{
            audio::_all::*,
            display::_all::*,
            // gpu::_all::*,
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
