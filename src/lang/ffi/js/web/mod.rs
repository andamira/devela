// devela::lang::ffi::js::web
//
//! <a href="https://developer.mozilla.org/en-US/docs/Web/API">Web APIs</a> interfacing.
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
            document::*, element::*, event::*, permission::*, web_api::*, window::*, worker::*,
        };
    }
}
