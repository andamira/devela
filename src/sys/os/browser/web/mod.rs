// devela/src/sys/os/browser/web/mod.rs
//
#![doc = crate::_DOC_SYS_OS_BROWSER_WEB!()] // public
#![doc = crate::_doc!(modules: crate::sys::os::browser; web)]
#![doc = crate::_doc!(flat:"sys")]
#![doc = crate::_doc!(hr)]
//!
//! - <https://developer.mozilla.org/en-US/docs/Web/API>.
//

mod time; // impls for JsInstant and JsTimeout

mod api; // Web (js & rust files)
mod document; // WebDocument, WebElement
#[cfg(feature = "event")]
#[cfg_attr(nightly_doc, doc(cfg(feature = "event")))]
mod event; // WebEventKind, WebEventMouse, WebEventPointer, WebKeyLocation
mod permission; // WebPermission, WebPermissionState
// mod url; // WebUrl, WebUrlSearchParams // WIP
mod window; // WebWindow
mod worker; // WebWorker, WebWorkerError, WebWorkerJob

crate::structural_mods! { // _mods, _crate_internals
    _mods {
        pub use super::{
            api::_all::*,
            document::_all::*,
            permission::*,
            window::*,
            worker::*,
        };
        #[cfg(feature = "event")]
        pub use super::event::_all::*;
    }
    _crate_internals {
    }
}
