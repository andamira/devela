// devela::lang::ffi::js::types
//
//! Defines the [`Js`] namespace and other items.
//

mod event; // JsEvent, JsEventMouse, JsEventPointer, JstKeyLocation
mod instant; // JsInstant
mod permission; // JsPermission, JsPermissionState
mod primitives; // js_number, js_int32, js_uint32...
mod text; // JsTextMetrics, JsTextMetricsFull
mod timeout; // JsTimeout
mod worker; // JsWorker, JsWorkerError, JsWorkerJob

pub use {event::*, instant::*, permission::*, primitives::*, text::*, timeout::*, worker::*};

// WIPZONE
// #[cfg(feature = "alloc")]
// crate::items! { mod bson; pub use bson::*; }
// #[cfg(feature = "std")]
// crate::items! { mod json; pub use json::*; }

#[doc = crate::TAG_NAMESPACE!()]
/// A Javascript namespace.
///
/// # Features
/// All methods depend on the `unsafe_ffi` feature and the `wasm32` architecture.
///
/// # Methods
/// - core APis
///   - [console](#web-api-console)
///   - [events](#web-api-events)
///   - [history](#web-api-history--location)
///   - [permissions](#web-api-permissions)
/// - extended APis
///   - media & graphics
///     - [canvas](#web-api-canvas)
//   - system & hardware
///   - performance & optimization
//     - time
//   - advanced & experimental
pub struct Js;
