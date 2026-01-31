// devela::sys::os::browser::web
//
#![doc = crate::_DOC_SYS_OS_BROWSER_WEB!()] // public
#![doc = crate::_doc!(modules: crate::sys::os::browser; web)]
#![doc = crate::_doc!(flat:"sys")]
#![doc = crate::_doc!(hr)]
//!
//! - <https://developer.mozilla.org/en-US/docs/Web/API>.
//

mod document; // WebDocument
mod element; // WebElement
mod event; // WebEventKind, WebEventMouse, WebEventPointer, WebKeyLocation
mod permission; // WebPermission, WebPermissionState
mod web_api; // Web
mod window; // WebWindow
mod worker; // WebWorker, WebWorkerError, WebWorkerJob

// WIPZONE
// mod url; // WebUrl, WebUrlSearchParams

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            document::*,
            element::*,
            event::*,
            permission::*,
            web_api::*,
            window::*,
            worker::*,
        };
    }
}
