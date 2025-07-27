// devela::lang::ffi::js::web::types
//
//! Defines Web API related types.
//

mod event; // JsEvent, JsEventMouse, JsEventPointer, JstKeyLocation
mod instant; // JsInstant
mod permission; // JsPermission, JsPermissionState
mod text; // JsTextMetrics, JsTextMetricsFull
mod timeout; // JsTimeout
mod worker; // JsWorker, JsWorkerError, JsWorkerJob

pub use {event::*, instant::*, permission::*, text::*, timeout::*, worker::*};

// WIPZONE
// #[cfg(feature = "alloc")]
// crate::items! { mod bson; pub use bson::*; }
// #[cfg(feature = "std")]
// crate::items! { mod json; pub use json::*; }

// mod document; // JsDocument, JsElement
// mod url; // JsUrl, JsUrlSearchParams
// mod window; // JsWindow
