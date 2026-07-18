// devela/src/sys/os/browser/web/mod.rs
//
#![doc = crate::_DOC_SYS_OS_BROWSER_WEB!()] // public
#![doc = crate::_doc!(modules: crate::sys::os::browser; web)]
#![doc = crate::_doc!(flat:"sys")]
#![doc = crate::_doc!(hr)]
//!
//! - <https://developer.mozilla.org/en-US/docs/Web/API>.
//

mod access; // permissions, credentials, clipboard authority
mod bridge; // Web (js & rust files)
// mod crypto; // Web Crypto
// mod device; // sensors, location, MIDI, gamepad, HID, USB…
#[cfg(feature = "event")]
#[cfg_attr(nightly_doc, doc(cfg(feature = "event")))]
mod event; // browser event representations and normalization
// mod graphics; // canvas, WebGl, WebGPU, image rendering
// mod media; // audio, video, capture, streams, recording
// mod network; // fetch, sockets, transports, RTC communication
mod page; // document, elements, navigation, screen, window
// mod resource; // URL, Blob, File, resource streams
// mod storage; // Web Storage, IndexedDB, Cache, origin filesystem
#[cfg(feature = "ui")]
#[cfg_attr(nightly_doc, doc(cfg(feature = "ui")))]
mod ui; // UI presenters implemented for the web
mod work; // clocks, frames, timers, workers, scheduling

crate::structural_mods! { // _mods, _crate_internals
    _mods {
        pub use super::{
            access::_all::*,
            bridge::_all::*,
            // crypto::_all::*,
            // device::_all::*,
            // graphics::_all::*,
            // media::_all::*,
            // network::_all::*,
            page::_all::*,
            // resource::_all::*,
            // storage::_all::*,
            work::_all::*,
        };
        #[cfg(feature = "event")]
        pub use super::event::_all::*;
        #[cfg(feature = "ui")]
        pub use super::ui::_all::*;
    }
    _crate_internals {}
}
