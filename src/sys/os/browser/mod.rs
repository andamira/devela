// devela::sys::os::browser
//
#![doc = crate::_DOC_SYS_OS_BROWSER!()] // public
#![doc = crate::_doc!(modules: crate::sys::os; browser)]
#![doc = crate::_doc!(flat:"sys")]
#![doc = crate::_doc!(hr)]
//!
//! Encompasses the full capability surface provided by browser runtimes,
//! including the standardized Web APIs (DOM, canvas, WebGL, audio, storage,
//! workers), as well as browser-specific extension interfaces, automation
//! hooks, and embedding environments. Represents all host-level facilities
//! available inside a browser sandbox.
//

// mod automation; // TODO
// mod container; // MAYBE container/bridge, hosted/integration
// mod extension; // WIP

#[cfg(all(feature = "js", not(windows)))]
#[cfg_attr(nightly_doc, doc(cfg(feature = "js")))]
pub mod web; // Web[Document|Element|Event*|Permission*|Window*|Worker*]…

crate::structural_mods! { // _pub_mods
    _pub_mods {
        #[cfg(all(feature = "js", not(windows)))]
        pub use super::web::_all::*;
    }
}
