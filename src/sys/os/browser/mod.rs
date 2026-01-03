// devela::sys::os::browser
//
//! Browser supervisory environment.
//!
//! Encompasses the full capability surface provided by browser runtimes,
//! including the standardized Web APIs (DOM, canvas, WebGL, audio, storage,
//! workers), as well as browser-specific extension interfaces, automation
//! hooks, and embedding environments. Represents all host-level facilities
//! available inside a browser sandbox.
//

#[cfg(all(feature = "js", not(windows)))]
#[cfg_attr(nightly_doc, doc(cfg(feature = "js")))]
pub mod web; // Web*

crate::structural_mods! { // _pub_mods
    _pub_mods {
        #[cfg(all(feature = "js", not(windows)))]
        pub use super::web::_all::*;
    }
}
