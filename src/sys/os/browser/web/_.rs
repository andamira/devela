// devela/src/sys/os/browser/web/_.rs
//
#![doc = crate::_DOC_SYS_OS_BROWSER_WEB!()] // public
#![doc = crate::_doc!(modules: crate::sys::os::browser; web)]
#![doc = crate::_doc!(flat:"sys")]
#![doc = crate::_doc!(hr)]
//!
//! - <https://developer.mozilla.org/en-US/docs/Web/API>.
//

crate::mods_in! {
    mod_ access; // permissions, credentials, clipboard authority
    mod_ bridge; // Web (js & rust files)
    // mod_ crypto; // Web Crypto
    // mod_ device; // sensors, location, MIDI, gamepad, HID, USB…
    #[cfg(feature = "event")]
    #[cfg_attr(nightly_doc, doc(cfg(feature = "event")))]
    mod_ event; // browser event representations and normalization
    // mod_ graphics; // canvas, WebGl, WebGPU, image rendering
    // mod_ media; // audio, video, capture, streams, recording
    // mod_ network; // fetch, sockets, transports, RTC communication
    mod_ page; // document, elements, navigation, screen, window
    // mod_ resource; // URL, Blob, File, resource streams
    // mod_ storage; // Web Storage, IndexedDB, Cache, origin filesystem
    #[cfg(feature = "ui")]
    #[cfg_attr(nightly_doc, doc(cfg(feature = "ui")))]
    mod_ ui; // UI presenters implemented for the web
    mod_ work; // clocks, frames, timers, workers, scheduling
}
crate::mods_out! { // _mods, _crate_internals
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
