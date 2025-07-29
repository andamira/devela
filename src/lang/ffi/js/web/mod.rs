// devela::lang::ffi::js::web
//
//! <a href="https://developer.mozilla.org/en-US/docs/Web/API">Web APIs</a> interfacing.

mod event; // WebEventKind, WebEventMouse, WebEventPointer, WebKeyLocation
mod permission; // WebPermission, WebPermissionState
mod web_api; // Web
mod window; // WebWindow
mod worker; // WebWorker, WebWorkerError, WebWorkerJob

crate::items! { // structural access: _mods, _all
    #[allow(unused)]
    pub use _mods::*;

    mod _mods { #![allow(unused)]
        pub use super::{event::*, permission::*, web_api::*, window::*, worker::*};
    }
    pub(super) mod _all { #![allow(unused)]
        #[doc(inline)]
        pub use super::_mods::*;
    }
}
// WIPZONE
// mod document; // WebDocument, WebElement
// mod url; // WebUrl, WebUrlSearchParams
// mod window; // WebWindow
